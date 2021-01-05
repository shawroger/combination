@Main 
{
    // Bind version
    @Bind($.version)("v0.1.3")
    @Join("Version: ")($.version)

    // Git to github
    @Println("Start deploy")
    @Println($.result)

    @Command("git add .")
    @Command("git commit -m $.version")
    @Command("git push origin master")

    @Println("Deploy is done")

    // publish to crates.index
    @Println("Publish to crates")
    @Bind($.url)("https://github.com/rust-lang/crates.io-index")
    @Command("cargo publish --index $.url")
    @Println("Publish is done")
}