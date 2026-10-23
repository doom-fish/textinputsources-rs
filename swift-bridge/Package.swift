// swift-tools-version:5.9
import PackageDescription

let package = Package(
  name: "TextInputSourcesBridge",
  platforms: [.macOS(.v12)],
  products: [
    .library(name: "TextInputSourcesBridge", type: .static, targets: ["TextInputSourcesBridge"])
  ],
  targets: [
    .target(
      name: "TextInputSourcesBridge", path: "Sources/TextInputSourcesBridge",
      publicHeadersPath: "include")
  ]
)
