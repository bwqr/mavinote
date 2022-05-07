import XCTest
@testable import Serde

final class SerdeTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(Serde().text, "Hello, World!")
    }
}
