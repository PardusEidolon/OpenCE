// The quality tiers: Low, Medium, High and Ultra, one picked from the Mac
// on first run (the GPU's core count, else its name's class, and the
// Mac's memory), each a preset of every graphics setting. A setting
// changed by hand makes the set Custom.

import Foundation
import IOKit
import Metal

/// A preset of the graphics settings.
public enum QualityTier: String, CaseIterable, Sendable {
    case low, medium, high, ultra

    public var title: String { rawValue.capitalized }

    /// The tier for a GPU of `cores` cores in a Mac of `memory` bytes. A
    /// Mac with less than 12 GB is held to Low whatever its GPU: the GPU
    /// may use about two thirds of an 8 GB Mac's memory, shared with the
    /// game and the system. Otherwise the cores: the base chips (7 to 10)
    /// aim at 1440p at 60, the Pro chips (14 to 19) at 1440p at 120 or 4K
    /// at 60, the Max and Ultra chips (24 and up) at 4K at 120.
    public static func pick(cores: Int, memory: UInt64) -> QualityTier {
        if memory < 12 << 30 { return .low }
        switch cores {
        case 24...: return .ultra
        case 14...: return .high
        case 10...: return .medium
        default: return .low
        }
    }

    /// The GPU's cores: the registry's count when there is one, else the
    /// fewest of the class its name gives ("Apple M2 Pro"), else 10 (an
    /// unknown GPU is taken as Medium's).
    public static func cores(name: String, registry: Int?) -> Int {
        if let registry, registry > 0 { return registry }
        let words = name.split(separator: " ")
        guard words.contains(where: { $0.hasPrefix("M") && $0.dropFirst().allSatisfy(\.isNumber) }) else {
            return 10
        }
        if words.contains("Ultra") { return 48 }
        if words.contains("Max") { return 24 }
        if words.contains("Pro") { return 14 }
        return 7
    }

    /// The tier for this Mac.
    public static func detect(_ device: MTLDevice) -> QualityTier {
        pick(cores: cores(name: device.name, registry: registryCores()), memory: ProcessInfo.processInfo.physicalMemory)
    }

    /// The GPU's core count from the I/O registry (the accelerator's
    /// `gpu-core-count`), or nil.
    static func registryCores() -> Int? {
        let service = IOServiceGetMatchingService(kIOMainPortDefault, IOServiceMatching("AGXAccelerator"))
        guard service != 0 else { return nil }
        defer { IOObjectRelease(service) }
        let value = IORegistryEntryCreateCFProperty(service, "gpu-core-count" as CFString, kCFAllocatorDefault, 0)
        return value?.takeRetainedValue() as? Int
    }
}

extension QualityTier {
    /// This Mac as the logs name it: the GPU, its cores and the memory
    /// ("Apple M1 Max, 32 GPU cores, 32 GB").
    public static func describe(_ device: MTLDevice) -> String {
        let cores = cores(name: device.name, registry: registryCores())
        return "\(device.name), \(cores) GPU cores, \(ProcessInfo.processInfo.physicalMemory >> 30) GB"
    }
}
