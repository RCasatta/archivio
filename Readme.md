## Archivio

[![crates.io](https://img.shields.io/crates/v/archivio.svg)](https://crates.io/crates/archivio)

When launched in a directory, it search contents of the Files sub-directory and creates a Tags directory with symbolic links to original content.
 
Files should follow the naming convention AAAA-MM-DD[_Tags]+

## Example

```
Files
├── 2019-10-13_Invoice_YCorp.pdf
└── 2020-11-12_Invoice_XCorp.pdf
```

will create

```
Tags/
├── 2019
│   └── 2019-10-13_Invoice_YCorp.pdf -> Files/2019-10-13_Invoice_YCorp.pdf
├── 2020
│   └── 2020-11-12_Invoice_XCorp.pdf -> Files/2020-11-12_Invoice_XCorp.pdf
├── Invoice
│   ├── 2019
│   │   └── 2019-10-13_Invoice_YCorp.pdf -> Files/2019-10-13_Invoice_YCorp.pdf
│   ├── 2019-10-13_Invoice_YCorp.pdf -> Files/2019-10-13_Invoice_YCorp.pdf
│   ├── 2020
│   │   └── 2020-11-12_Invoice_XCorp.pdf -> Files/2020-11-12_Invoice_XCorp.pdf
│   ├── 2020-11-12_Invoice_XCorp.pdf -> Files/2020-11-12_Invoice_XCorp.pdf
│   ├── XCorp
│   │   └── 2020-11-12_Invoice_XCorp.pdf -> Files/2020-11-12_Invoice_XCorp.pdf
│   └── YCorp
│       └── 2019-10-13_Invoice_YCorp.pdf -> Files/2019-10-13_Invoice_YCorp.pdf
├── XCorp
│   └── 2020-11-12_Invoice_XCorp.pdf -> Files/2020-11-12_Invoice_XCorp.pdf
└── YCorp
    └── 2019-10-13_Invoice_YCorp.pdf -> Files/2019-10-13_Invoice_YCorp.pdf
```


