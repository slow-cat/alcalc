import XCTest
import SwiftTreeSitter
import TreeSitterAlg

final class TreeSitterAlgTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_alg())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Alg grammar")
    }
}
