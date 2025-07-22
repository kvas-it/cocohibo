# Test Data for Cocohibo

This directory contains sample projects and chats for testing Cocohibo's parsing and display functionality.

## Structure

```
tests/
└── sample-projects/
    ├── test-project-1/
    │   ├── basic-conversation.jsonl
    │   └── tool-usage-example.jsonl
    └── debugging-session/
        └── error-investigation.jsonl
```

## Test Projects

### test-project-1
Contains examples of:
- Basic user/assistant conversation
- Tool usage with thinking blocks
- Various message types and metadata
- Standard token usage patterns

### debugging-session
Contains examples of:
- Meta messages (`isMeta: true`)
- Sidechain messages (`isSidechain: true`)
- Different user types (`internal` vs `external`)
- Beta service tier usage
- Different git branches and working directories

## Message Types Covered

The test data includes examples of:
- User messages with simple text content
- Assistant messages with structured content arrays
- Messages with thinking blocks
- Tool use and tool result messages
- Messages with various metadata fields
- Different service tiers and usage patterns
- Cache usage statistics

## Usage

To test with this data, run cocohibo with the test directory:

```bash
cargo run --bin cocohibo -- --projects-dir tests/sample-projects
```

## Adding New Test Cases

When adding new test cases:
1. Create realistic message structures based on actual Claude Code output
2. Include edge cases that might cause parsing issues
3. Test different combinations of metadata fields
4. Document any specific parsing challenges the test case addresses