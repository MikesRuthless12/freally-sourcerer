cask "freally-sourcerer" do
  arch arm: "aarch64", intel: "x64"

  version "0.23.2"
  sha256 arm:   "33b82573ec013a7c38f705a31f2d47f9dacb0955adb6e724c6cc0daa1581aee2",
         intel: "dd2f9c1d660a0585fff51fbc7682326e73a3160aafe95f21d2d22c5fff999b67"

  url "https://github.com/MikesRuthless12/freally-sourcerer/releases/download/v#{version}/Freally.Sourcerer_#{version}_#{arch}.dmg",
      verified: "github.com/MikesRuthless12/freally-sourcerer/"
  name "Freally Sourcerer"
  desc "One search. Every source. Every OS."
  homepage "https://github.com/MikesRuthless12/freally-sourcerer"

  livecheck do
    url :url
    strategy :github_latest
  end

  depends_on macos: ">= :monterey"

  app "Freally Sourcerer.app"

  # The indexer is a separate long-lived process with its own per-user
  # state. Leaving it running after an uninstall keeps a stale index alive
  # and a LaunchAgent pointing at a binary that is gone.
  uninstall quit:      "io.mikeweaver.freally",
            launchctl: "io.mikeweaver.freally.indexd"

  zap trash: [
    "~/Library/Application Support/freally",
    "~/Library/Caches/io.mikeweaver.freally",
    "~/Library/LaunchAgents/io.mikeweaver.freally.indexd.plist",
    "~/Library/Preferences/io.mikeweaver.freally.plist",
    "~/Library/Saved Application State/io.mikeweaver.freally.savedState",
  ]
end
