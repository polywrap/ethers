// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "MetamaskProvider",
    platforms: [.iOS(.v15)],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "MetamaskProvider",
            targets: ["MetamaskProvider"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(name: "PolywrapClient", path: "../../../../swift-client"),
        .package(name: "metamask-ios-sdk", path: "../../../../../metamask/ios-sdk")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "MetamaskProvider",
            dependencies: [
                "PolywrapClient",
                "metamask-ios-sdk"
            ],
            cSettings: [
                .headerSearchPath("../../include")
            ]
        ),
    ],
    swiftLanguageVersions: [.v5]
)
