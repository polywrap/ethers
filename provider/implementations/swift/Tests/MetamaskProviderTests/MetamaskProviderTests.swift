import XCTest
@testable import implementations

final class implementationsTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(implementations().text, "Hello, World!")
    }
}
