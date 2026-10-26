// Dynamic resolution: the render scale is stepped down when the GPU's
// frames run long and back up when the step above is predicted to fit,
// between a few fixed steps whose scalers and targets are made at load
// (a step costs memory, about 59 bytes a pixel of its input at 4K, so
// three steps are kept: the maximum, 0.67 and 0.5).

import Foundation

/// The controller that picks the render scale from the GPU's frame times.
public struct DynamicResolution: Equatable, Sendable {
    /// The frame time held to, in milliseconds.
    public let targetMilliseconds: Double
    /// The scales stepped between, largest first.
    public let steps: [Float]
    /// The step drawn at (0 is the largest).
    public private(set) var step = 0

    /// The frames averaged before each decision.
    static let window = 30
    /// The frames left alone after a change (the new size's first frames
    /// are its scaler's reset, and the times still in flight are the old
    /// size's).
    static let settle = 30
    /// A step down when the mean is above this share of the target.
    static let high = 0.9
    /// A step up when the step above is predicted below this share.
    static let low = 0.75
    /// The steps below the maximum.
    static let lower: [Float] = [0.67, 0.5]

    private var sum = 0.0
    private var count = 0
    private var settling = 0
    /// The mean before the last change, and the step it was measured at.
    private var before: Measured?
    /// The measured cost of the step above each step over its own, from
    /// the last change between them.
    private var upRatios: [Double?]

    /// A controller that holds `targetMilliseconds`, starting at
    /// `maximum` (clamped to the render scales).
    public init(targetMilliseconds: Double, maximum: Float) {
        let range = VideoSettings.renderScales
        let top = maximum.isFinite ? min(max(maximum, range.lowerBound), range.upperBound) : 1
        self.targetMilliseconds = targetMilliseconds
        steps = [top] + Self.lower.filter { $0 < top - 0.01 }
        upRatios = Array(repeating: nil, count: steps.count)
    }

    /// The scale drawn at.
    public var scale: Float { steps.indices.contains(step) ? steps[step] : 1 }

    /// The share of the area the step above draws over this one's.
    private func areaRatio(_ step: Int) -> Double {
        let ratio = Double(steps[step - 1] / steps[step])
        return ratio * ratio
    }

    /// Takes one frame's GPU time; true when the scale changed. Times
    /// that are not finite or not above 0 are ignored.
    public mutating func record(milliseconds: Double) -> Bool {
        guard milliseconds.isFinite, milliseconds > 0 else { return false }
        if settling > 0 {
            settling -= 1
            return false
        }
        sum += milliseconds
        count += 1
        guard count >= Self.window else { return false }
        let mean = sum / Double(count)
        sum = 0
        count = 0
        if let before {
            // (the cost of the upper step over the lower, measured across
            // the change)
            if before.step == step - 1 {
                upRatios[step] = before.mean / mean
            } else if before.step == step + 1 {
                upRatios[before.step] = mean / before.mean
            }
            self.before = nil
        }
        if mean > Self.high * targetMilliseconds, step + 1 < steps.count {
            change(to: step + 1, mean: mean)
            return true
        }
        if step > 0 {
            let area = areaRatio(step)
            let ratio = min(max(upRatios[step] ?? area, 1), area)
            if mean * ratio < Self.low * targetMilliseconds {
                change(to: step - 1, mean: mean)
                return true
            }
        }
        return false
    }

    private mutating func change(to next: Int, mean: Double) {
        before = Measured(step: step, mean: mean)
        step = next
        settling = Self.settle
    }

    private struct Measured: Equatable, Sendable {
        let step: Int
        let mean: Double
    }
}

/// The GPU's frame times, gathered from the command buffers' completion
/// handlers and drained by the next frame's encode: each is the buffer's
/// busy time, or the time since the last buffer ended when that is
/// shorter (the buffers in flight overlap).
final class GpuFrameTimes: @unchecked Sendable {
    private let lock = NSLock()
    private var lastEnd: CFTimeInterval?
    /// (fixed: a frame drains it, and a full buffer drops the oldest)
    private var times = [Double](repeating: 0, count: 8)
    private var count = 0

    func completed(start: CFTimeInterval, end: CFTimeInterval) {
        lock.lock()
        defer { lock.unlock() }
        var time = (end - start) * 1000
        if let lastEnd { time = min(time, (end - lastEnd) * 1000) }
        lastEnd = end
        add(locked: time)
    }

    /// Adds a time measured elsewhere (for tests).
    func add(milliseconds: Double) {
        lock.lock()
        defer { lock.unlock() }
        add(locked: milliseconds)
    }

    private func add(locked time: Double) {
        if count == times.count {
            times.withUnsafeMutableBufferPointer { buffer in
                for index in 1..<buffer.count { buffer[index - 1] = buffer[index] }
            }
            count -= 1
        }
        times[count] = time
        count += 1
    }

    /// Hands each time gathered to `body`, oldest first, and forgets them
    /// (`body` is called with the lock held: it must not call back).
    func drain(_ body: (Double) -> Void) {
        lock.lock()
        defer { lock.unlock() }
        for index in 0..<count { body(times[index]) }
        count = 0
    }

    /// Forgets the times and the last end (the frames drawn next are a
    /// new size's).
    func reset() {
        lock.lock()
        defer { lock.unlock() }
        count = 0
        lastEnd = nil
    }
}
