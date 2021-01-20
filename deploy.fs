@Main 
{
    // Bind version
    @Bind($.version)("0.1.5")
    @Join("Version: ")($.version)

    // Git to github
    @Println("Start deploy")
    @Println($.result)

    @Command("git add .")
    @Command("git commit -m $.version")
    @Command("git push origin master")

    @Println("Deploy is done")
    @Println("")

    // publish to crates.index
    @Println("Publish to crates")
    @Bind($.url)("https://github.com/rust-lang/crates.io-index")
    @File("E:\Rust\.cargo\token.txt")
    {
        @BindRoot($.token)($.content)
    }
    @Command("cargo publish --index $.url --token $.token")
    
    @Println("Publish is done")
}