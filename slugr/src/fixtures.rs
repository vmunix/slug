//! Test fixtures — wild filenames from the real world.
//!
//! Each fixture tests `slugify()` across all three naming styles (kebab, snake, camel).
//! Run with: cargo test fixtures

#[cfg(test)]
mod tests {
    use fileslug::{slugify, SlugifyOptions, Style};

    macro_rules! fixture_tests {
        ($(
            $name:ident: $input:expr =>
                kebab: $kebab:expr,
                snake: $snake:expr,
                camel: $camel:expr;
        )*) => {
            $(
                mod $name {
                    use super::*;

                    #[test]
                    fn kebab() {
                        let opts = SlugifyOptions { style: Style::Kebab, ..Default::default() };
                        assert_eq!(
                            slugify($input, &opts), $kebab,
                            "\n  input: {:?}\n  style: kebab", $input
                        );
                    }

                    #[test]
                    fn snake() {
                        let opts = SlugifyOptions { style: Style::Snake, ..Default::default() };
                        assert_eq!(
                            slugify($input, &opts), $snake,
                            "\n  input: {:?}\n  style: snake", $input
                        );
                    }

                    #[test]
                    fn camel() {
                        let opts = SlugifyOptions { style: Style::Camel, ..Default::default() };
                        assert_eq!(
                            slugify($input, &opts), $camel,
                            "\n  input: {:?}\n  style: camel", $input
                        );
                    }
                }
            )*
        };
    }

    fixture_tests! {
        // =====================================================================
        // Real-world messy — the Downloads folder hall of shame
        // =====================================================================

        download_duplicate: "Report (2) (copy).pdf" =>
            kebab: "report-2-copy.pdf",
            snake: "report_2_copy.pdf",
            camel: "report2Copy.pdf";

        macos_screenshot: "Screenshot 2024-03-01 at 2.15.32\u{202f}PM.png" =>
            kebab: "screenshot-2024-03-01-at-2.15.32-pm.png",
            snake: "screenshot_2024_03_01_at_2.15.32_pm.png",
            camel: "screenshot20240301At2.15.32Pm.png";

        camera_roll: "IMG_20240301_142359.jpg" =>
            kebab: "img-20240301-142359.jpg",
            snake: "img_20240301_142359.jpg",
            camel: "img20240301142359.jpg";

        raw_photo_dupe: "DSC_0042 (1).NEF" =>
            kebab: "dsc-0042-1.NEF",
            snake: "dsc_0042_1.NEF",
            camel: "dsc00421.NEF";

        music_track: "01 - Artist - Track Name (feat. Other Artist).mp3" =>
            kebab: "01-artist-track-name-feat-other-artist.mp3",
            snake: "01_artist_track_name_feat_other_artist.mp3",
            camel: "01ArtistTrackNameFeatOtherArtist.mp3";

        final_v2: "my_document - final FINAL (really final) v2.docx" =>
            kebab: "my-document-final-final-really-final-v2.docx",
            snake: "my_document_final_final_really_final_v2.docx",
            camel: "myDocumentFinalFinalReallyFinalV2.docx";

        copy_of_copy: "Copy of Copy of Budget_2024 (1).xlsx" =>
            kebab: "copy-of-copy-of-budget-2024-1.xlsx",
            snake: "copy_of_copy_of_budget_2024_1.xlsx",
            camel: "copyOfCopyOfBudget20241.xlsx";

        brackets_mixed: "Document(1) [Reviewed].pdf" =>
            kebab: "document-1-reviewed.pdf",
            snake: "document_1_reviewed.pdf",
            camel: "document1Reviewed.pdf";

        scanned_tax: "tax return 2023 - SIGNED (scanned).PDF" =>
            kebab: "tax-return-2023-signed-scanned.PDF",
            snake: "tax_return_2023_signed_scanned.PDF",
            camel: "taxReturn2023SignedScanned.PDF";

        word_temp: "~$word temp file.docx" =>
            kebab: "word-temp-file.docx",
            snake: "word_temp_file.docx",
            camel: "wordTempFile.docx";

        photoshop_copy: "Untitled-1 (3rd copy).psd" =>
            kebab: "untitled-1-3rd-copy.psd",
            snake: "untitled_1_3rd_copy.psd",
            camel: "untitled13rdCopy.psd";

        download_hell: "index (1) (1) (2).html" =>
            kebab: "index-1-1-2.html",
            snake: "index_1_1_2.html",
            camel: "index112.html";

        // =====================================================================
        // Unicode & international — the world is bigger than ASCII
        // =====================================================================

        spanish_tilde: "Ñoño café.txt" =>
            kebab: "nono-cafe.txt",
            snake: "nono_cafe.txt",
            camel: "nonoCafe.txt";

        german_umlauts: "Ünïcödé Fïlé.txt" =>
            kebab: "unicode-file.txt",
            snake: "unicode_file.txt",
            camel: "unicodeFile.txt";

        russian_cyrillic: "\u{041F}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442} \u{043C}\u{0438}\u{0440}.txt" =>
            kebab: "privet-mir.txt",
            snake: "privet_mir.txt",
            camel: "privetMir.txt";

        japanese_mixed: "\u{6771}\u{4EAC}\u{30BF}\u{30EF}\u{30FC}.jpg" =>
            kebab: "dongjingtawa.jpg",
            snake: "dongjingtawa.jpg",
            camel: "dongjingtawa.jpg";

        arabic_rtl: "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{0628}\u{0627}\u{0644}\u{0639}\u{0627}\u{0644}\u{0645}.txt" =>
            kebab: "mrhb-bl-lm.txt",
            snake: "mrhb_bl_lm.txt",
            camel: "mrhbBlLm.txt";

        emoji_overload: "\u{1F389} Party \u{1F382} Time \u{1F973}.txt" =>
            kebab: "tada-party-birthday-time-partying-face.txt",
            snake: "tada_party_birthday_time_partying_face.txt",
            camel: "tadaPartyBirthdayTimePartyingFace.txt";

        french_cedilla: "Ça fait beau temps.txt" =>
            kebab: "ca-fait-beau-temps.txt",
            snake: "ca_fait_beau_temps.txt",
            camel: "caFaitBeauTemps.txt";

        mixed_scripts: "Tokyo \u{6771}\u{4EAC} 2024.txt" =>
            kebab: "tokyo-dongjing-2024.txt",
            snake: "tokyo_dongjing_2024.txt",
            camel: "tokyoDongjing2024.txt";

        nordic_french: "Ångström naïve.txt" =>
            kebab: "angstrom-naive.txt",
            snake: "angstrom_naive.txt",
            camel: "angstromNaive.txt";

        zero_width_chars: "hello\u{200B}world\u{FEFF}file.txt" =>
            kebab: "helloworldfile.txt",
            snake: "helloworldfile.txt",
            camel: "helloworldfile.txt";

        // =====================================================================
        // Shell / security adversarial — should produce safe filenames
        // =====================================================================

        cmd_substitution: "$(rm -rf /).txt" =>
            kebab: "rm-rf.txt",
            snake: "rm_rf.txt",
            camel: "rmRf.txt";

        backtick_inject: "`whoami`.txt" =>
            kebab: "whoami.txt",
            snake: "whoami.txt",
            camel: "whoami.txt";

        semicolon_chain: "file;rm -rf /;.txt" =>
            kebab: "file-rm-rf.txt",
            snake: "file_rm_rf.txt",
            camel: "fileRmRf.txt";

        quotes_mixed: "it's \"my\" file.txt" =>
            kebab: "it-s-my-file.txt",
            snake: "it_s_my_file.txt",
            camel: "itSMyFile.txt";

        xss_attempt: "<script>alert(1)</script>.html" =>
            kebab: "script-alert-1-script.html",
            snake: "script_alert_1_script.html",
            camel: "scriptAlert1Script.html";

        ampersand_chain: "file && echo pwned.txt" =>
            kebab: "file-echo-pwned.txt",
            snake: "file_echo_pwned.txt",
            camel: "fileEchoPwned.txt";

        env_expansion: "${HOME}/.secret.txt" =>
            kebab: "home-secret.txt",
            snake: "home_secret.txt",
            camel: "homeSecret.txt";

        backslash_path: "path\\to\\file.txt" =>
            kebab: "path-to-file.txt",
            snake: "path_to_file.txt",
            camel: "pathToFile.txt";

        // =====================================================================
        // Filesystem edge cases
        // =====================================================================

        mixed_separators: "file--name__also  here.txt" =>
            kebab: "file-name-also-here.txt",
            snake: "file_name_also_here.txt",
            camel: "fileNameAlsoHere.txt";

        windows_reserved: "CON.txt" =>
            kebab: "con.txt",
            snake: "con.txt",
            camel: "con.txt";

        compound_with_extras: "archive.backup.2024.tar.gz" =>
            kebab: "archive-backup-2024.tar.gz",
            snake: "archive_backup_2024.tar.gz",
            camel: "archiveBackup2024.tar.gz";

        no_extension_dir: "NEW FOLDER (2)" =>
            kebab: "new-folder-2",
            snake: "new_folder_2",
            camel: "newFolder2";

        dotfile_with_spaces: ". hidden but spaces .txt" =>
            kebab: ".hidden-but-spaces.txt",
            snake: ".hidden_but_spaces.txt",
            camel: ".hiddenButSpaces.txt";

        many_dots: "a.b.c.d.e.f.g.txt" =>
            kebab: "a-b-c-d-e-f-g.txt",
            snake: "a_b_c_d_e_f_g.txt",
            camel: "aBCDEFG.txt";

        all_caps: "ALL CAPS SHOUTING FILE.TXT" =>
            kebab: "all-caps-shouting-file.TXT",
            snake: "all_caps_shouting_file.TXT",
            camel: "allCapsShoutingFile.TXT";

        all_dots: "..." =>
            kebab: ".",
            snake: ".",
            camel: ".";

        // =====================================================================
        // Version number preservation — dots in version strings stay as dots
        // =====================================================================

        semver_dmg: "monarch-0.8.34.dmg" =>
            kebab: "monarch-0.8.34.dmg",
            snake: "monarch_0.8.34.dmg",
            camel: "monarch0.8.34.dmg";

        version_two_part: "Shottr-1.9.dmg" =>
            kebab: "shottr-1.9.dmg",
            snake: "shottr_1.9.dmg",
            camel: "shottr1.9.dmg";

        version_glued_to_name: "iStatMenus7.20.zip" =>
            kebab: "istatmenus7.20.zip",
            snake: "istatmenus7.20.zip",
            camel: "istatmenus7.20.zip";

        version_three_part: "NetNewsWire6.2.1.zip" =>
            kebab: "netnewswire6.2.1.zip",
            snake: "netnewswire6.2.1.zip",
            camel: "netnewswire6.2.1.zip";

        version_underscored: "Antinote_1.1.7.dmg" =>
            kebab: "antinote-1.1.7.dmg",
            snake: "antinote_1.1.7.dmg",
            camel: "antinote1.1.7.dmg";

        version_multiple: "Launch Schedule 2.10-2.12.26.png" =>
            kebab: "launch-schedule-2.10-2.12.26.png",
            snake: "launch_schedule_2.10_2.12.26.png",
            camel: "launchSchedule2.102.12.26.png";
    }
}
