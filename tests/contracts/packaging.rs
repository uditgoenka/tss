fn file(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn npm_package_exposes_tss_binary_without_reimplementing_the_cli() {
    let manifest = file("package.json");
    let wrapper = file("npm/bin/tss");
    let postinstall = file("npm/postinstall.js");

    assert!(manifest.contains("\"bin\""));
    assert!(manifest.contains("\"tss\": \"npm/bin/tss\""));
    assert!(manifest.contains("\"license\": \"Apache-2.0\""));
    assert!(manifest.contains("\"postinstall\": \"node npm/postinstall.js\""));
    assert!(wrapper.contains("TSS_BINARY"));
    assert!(wrapper.contains("spawnSync"));
    assert!(postinstall.contains("github.com/uditgoenka/tss"));
    assert!(postinstall.contains("TSS_SKIP_DOWNLOAD"));
    assert!(postinstall.contains("checksums.json"));
    assert!(postinstall.contains("createHash"));
    assert!(postinstall.contains("trustedHosts"));
    assert!(postinstall.contains("COPYFILE_EXCL"));
    assert!(postinstall.contains("objects.githubusercontent.com"));
    assert!(file("npm/checksums.json").contains("tss-0.1.01-darwin-arm64"));
}

#[test]
fn homebrew_template_builds_tagged_source_with_cargo() {
    let formula = file("packaging/homebrew/tss.rb.template");

    assert!(formula.contains("class Tss < Formula"));
    assert!(formula.contains("Apache-2.0"));
    assert!(formula.contains("github.com/uditgoenka/tss"));
    assert!(formula.contains("archive/refs/tags"));
    assert!(formula.contains("version \"{{version}}\""));
    assert!(formula.contains("depends_on \"rust\" => :build"));
    assert!(formula.contains("cargo"));
    assert!(formula.contains("system \"#{bin}/tss\", \"--version\""));
}

#[test]
fn readme_documents_distribution_and_trust_contract() {
    let readme = file("README.md");

    for expected in [
        "Token Saving Scheme",
        "npm install",
        "brew install",
        "tss run --",
        "tss proxy",
        "tss raw",
        "Apache-2.0",
        "Migration",
        "TSS vs RTK",
        "evals.md",
        "150/150 local eval iterations passed",
        "139.7K estimated tokens",
        "Pi.dev",
        "paypal.me/uditgoenka",
        "<h2 align=\"center\">Contributor</h2>",
        "github.com/uditgoenka\">uditgoenka</a>",
    ] {
        assert!(
            readme.contains(expected),
            "missing README entry: {expected}"
        );
    }

    let blocked_phrases = [
        ["RTK", " parity"].concat(),
        ["RTK", " issue classes"].concat(),
        ["udit", "-", "fs"].concat(),
        ["first", "sales"].concat(),
    ];

    for blocked in blocked_phrases {
        assert!(
            !readme.contains(&blocked),
            "README contains blocked phrase: {blocked}"
        );
    }
}
