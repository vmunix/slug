//! Test fixtures — wild filenames from the real world.
//!
//! Each fixture tests `slugify()` across all three naming styles (kebab, snake, pascal).
//! Run with: cargo test fixtures

#[cfg(test)]
mod tests {
    use fileslug::{slugify, SlugifyOptions, Style};

    macro_rules! fixture_tests {
        ($(
            $name:ident: $input:expr =>
                kebab: $kebab:expr,
                snake: $snake:expr,
                pascal: $pascal:expr;
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
                    fn pascal() {
                        let opts = SlugifyOptions { style: Style::Pascal, ..Default::default() };
                        assert_eq!(
                            slugify($input, &opts), $pascal,
                            "\n  input: {:?}\n  style: pascal", $input
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
            pascal: "Report2Copy.pdf";

        macos_screenshot: "Screenshot 2024-03-01 at 2.15.32\u{202f}PM.png" =>
            kebab: "screenshot-2024-03-01-at-2.15.32-pm.png",
            snake: "screenshot_2024_03_01_at_2.15.32_pm.png",
            pascal: "Screenshot20240301At2.15.32Pm.png";

        camera_roll: "IMG_20240301_142359.jpg" =>
            kebab: "img-20240301-142359.jpg",
            snake: "img_20240301_142359.jpg",
            pascal: "Img20240301142359.jpg";

        raw_photo_dupe: "DSC_0042 (1).NEF" =>
            kebab: "dsc-0042-1.NEF",
            snake: "dsc_0042_1.NEF",
            pascal: "Dsc00421.NEF";

        music_track: "01 - Artist - Track Name (feat. Other Artist).mp3" =>
            kebab: "01-artist-track-name-feat-other-artist.mp3",
            snake: "01_artist_track_name_feat_other_artist.mp3",
            pascal: "01ArtistTrackNameFeatOtherArtist.mp3";

        final_v2: "my_document - final FINAL (really final) v2.docx" =>
            kebab: "my-document-final-final-really-final-v2.docx",
            snake: "my_document_final_final_really_final_v2.docx",
            pascal: "MyDocumentFinalFinalReallyFinalV2.docx";

        copy_of_copy: "Copy of Copy of Budget_2024 (1).xlsx" =>
            kebab: "copy-of-copy-of-budget-2024-1.xlsx",
            snake: "copy_of_copy_of_budget_2024_1.xlsx",
            pascal: "CopyOfCopyOfBudget20241.xlsx";

        brackets_mixed: "Document(1) [Reviewed].pdf" =>
            kebab: "document-1-reviewed.pdf",
            snake: "document_1_reviewed.pdf",
            pascal: "Document1Reviewed.pdf";

        scanned_tax: "tax return 2023 - SIGNED (scanned).PDF" =>
            kebab: "tax-return-2023-signed-scanned.PDF",
            snake: "tax_return_2023_signed_scanned.PDF",
            pascal: "TaxReturn2023SignedScanned.PDF";

        word_temp: "~$word temp file.docx" =>
            kebab: "word-temp-file.docx",
            snake: "word_temp_file.docx",
            pascal: "WordTempFile.docx";

        photoshop_copy: "Untitled-1 (3rd copy).psd" =>
            kebab: "untitled-1-3rd-copy.psd",
            snake: "untitled_1_3rd_copy.psd",
            pascal: "Untitled13rdCopy.psd";

        download_hell: "index (1) (1) (2).html" =>
            kebab: "index-1-1-2.html",
            snake: "index_1_1_2.html",
            pascal: "Index112.html";

        // =====================================================================
        // Unicode & international — the world is bigger than ASCII
        // =====================================================================

        spanish_tilde: "Ñoño café.txt" =>
            kebab: "nono-cafe.txt",
            snake: "nono_cafe.txt",
            pascal: "NonoCafe.txt";

        german_umlauts: "Ünïcödé Fïlé.txt" =>
            kebab: "unicode-file.txt",
            snake: "unicode_file.txt",
            pascal: "UnicodeFile.txt";

        russian_cyrillic: "\u{041F}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442} \u{043C}\u{0438}\u{0440}.txt" =>
            kebab: "privet-mir.txt",
            snake: "privet_mir.txt",
            pascal: "PrivetMir.txt";

        japanese_mixed: "\u{6771}\u{4EAC}\u{30BF}\u{30EF}\u{30FC}.jpg" =>
            kebab: "dongjingtawa.jpg",
            snake: "dongjingtawa.jpg",
            pascal: "Dongjingtawa.jpg";

        arabic_rtl: "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{0628}\u{0627}\u{0644}\u{0639}\u{0627}\u{0644}\u{0645}.txt" =>
            kebab: "mrhb-bl-lm.txt",
            snake: "mrhb_bl_lm.txt",
            pascal: "MrhbBlLm.txt";

        emoji_overload: "\u{1F389} Party \u{1F382} Time \u{1F973}.txt" =>
            kebab: "tada-party-birthday-time-partying-face.txt",
            snake: "tada_party_birthday_time_partying_face.txt",
            pascal: "TadaPartyBirthdayTimePartyingFace.txt";

        french_cedilla: "Ça fait beau temps.txt" =>
            kebab: "ca-fait-beau-temps.txt",
            snake: "ca_fait_beau_temps.txt",
            pascal: "CaFaitBeauTemps.txt";

        mixed_scripts: "Tokyo \u{6771}\u{4EAC} 2024.txt" =>
            kebab: "tokyo-dongjing-2024.txt",
            snake: "tokyo_dongjing_2024.txt",
            pascal: "TokyoDongjing2024.txt";

        nordic_french: "Ångström naïve.txt" =>
            kebab: "angstrom-naive.txt",
            snake: "angstrom_naive.txt",
            pascal: "AngstromNaive.txt";

        zero_width_chars: "hello\u{200B}world\u{FEFF}file.txt" =>
            kebab: "helloworldfile.txt",
            snake: "helloworldfile.txt",
            pascal: "Helloworldfile.txt";

        // =====================================================================
        // Shell / security adversarial — should produce safe filenames
        // =====================================================================

        cmd_substitution: "$(rm -rf /).txt" =>
            kebab: "rm-rf.txt",
            snake: "rm_rf.txt",
            pascal: "RmRf.txt";

        backtick_inject: "`whoami`.txt" =>
            kebab: "whoami.txt",
            snake: "whoami.txt",
            pascal: "Whoami.txt";

        semicolon_chain: "file;rm -rf /;.txt" =>
            kebab: "file-rm-rf.txt",
            snake: "file_rm_rf.txt",
            pascal: "FileRmRf.txt";

        quotes_mixed: "it's \"my\" file.txt" =>
            kebab: "it-s-my-file.txt",
            snake: "it_s_my_file.txt",
            pascal: "ItSMyFile.txt";

        xss_attempt: "<script>alert(1)</script>.html" =>
            kebab: "script-alert-1-script.html",
            snake: "script_alert_1_script.html",
            pascal: "ScriptAlert1Script.html";

        ampersand_chain: "file && echo pwned.txt" =>
            kebab: "file-echo-pwned.txt",
            snake: "file_echo_pwned.txt",
            pascal: "FileEchoPwned.txt";

        env_expansion: "${HOME}/.secret.txt" =>
            kebab: "home-secret.txt",
            snake: "home_secret.txt",
            pascal: "HomeSecret.txt";

        backslash_path: "path\\to\\file.txt" =>
            kebab: "path-to-file.txt",
            snake: "path_to_file.txt",
            pascal: "PathToFile.txt";

        // =====================================================================
        // Filesystem edge cases
        // =====================================================================

        mixed_separators: "file--name__also  here.txt" =>
            kebab: "file-name-also-here.txt",
            snake: "file_name_also_here.txt",
            pascal: "FileNameAlsoHere.txt";

        windows_reserved: "CON.txt" =>
            kebab: "con.txt",
            snake: "con.txt",
            pascal: "Con.txt";

        compound_with_extras: "archive.backup.2024.tar.gz" =>
            kebab: "archive-backup-2024.tar.gz",
            snake: "archive_backup_2024.tar.gz",
            pascal: "ArchiveBackup2024.tar.gz";

        no_extension_dir: "NEW FOLDER (2)" =>
            kebab: "new-folder-2",
            snake: "new_folder_2",
            pascal: "NewFolder2";

        dotfile_with_spaces: ". hidden but spaces .txt" =>
            kebab: ".hidden-but-spaces.txt",
            snake: ".hidden_but_spaces.txt",
            pascal: ".HiddenButSpaces.txt";

        many_dots: "a.b.c.d.e.f.g.txt" =>
            kebab: "a-b-c-d-e-f-g.txt",
            snake: "a_b_c_d_e_f_g.txt",
            pascal: "ABCDEFG.txt";

        all_caps: "ALL CAPS SHOUTING FILE.TXT" =>
            kebab: "all-caps-shouting-file.TXT",
            snake: "all_caps_shouting_file.TXT",
            pascal: "AllCapsShoutingFile.TXT";

        all_dots: "..." =>
            kebab: ".",
            snake: ".",
            pascal: ".";

        // =====================================================================
        // Version number preservation — dots in version strings stay as dots
        // =====================================================================

        semver_dmg: "monarch-0.8.34.dmg" =>
            kebab: "monarch-0.8.34.dmg",
            snake: "monarch_0.8.34.dmg",
            pascal: "Monarch0.8.34.dmg";

        version_two_part: "Shottr-1.9.dmg" =>
            kebab: "shottr-1.9.dmg",
            snake: "shottr_1.9.dmg",
            pascal: "Shottr1.9.dmg";

        version_glued_to_name: "iStatMenus7.20.zip" =>
            kebab: "istatmenus7.20.zip",
            snake: "istatmenus7.20.zip",
            pascal: "Istatmenus7.20.zip";

        version_three_part: "NetNewsWire6.2.1.zip" =>
            kebab: "netnewswire6.2.1.zip",
            snake: "netnewswire6.2.1.zip",
            pascal: "Netnewswire6.2.1.zip";

        version_underscored: "Antinote_1.1.7.dmg" =>
            kebab: "antinote-1.1.7.dmg",
            snake: "antinote_1.1.7.dmg",
            pascal: "Antinote1.1.7.dmg";

        version_multiple: "Launch Schedule 2.10-2.12.26.png" =>
            kebab: "launch-schedule-2.10-2.12.26.png",
            snake: "launch_schedule_2.10_2.12.26.png",
            pascal: "LaunchSchedule2.102.12.26.png";
    }
}
