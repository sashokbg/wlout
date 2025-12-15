use clap::{ArgMatches, Command};
use clap_complete::{Generator, Shell, generate};
use std::io::{self, Write};

struct AdvancedZsh {}

impl Generator for AdvancedZsh {
    fn file_name(&self, name: &str) -> String {
        Shell::Zsh.file_name(name)
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        let mut temp_buffer = Vec::new();
        generate(
            Shell::Zsh,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut temp_buffer,
        );

        let mut script =
            String::from_utf8(temp_buffer).expect("completion script should be valid UTF-8");

        script.push_str(
            r#"
_wlout_list_displays() {
    local output
    local -a displays
    output=$(command wlout list 2>/dev/null) || return 0
    displays=(${(s:\n:)${${output//$'\t'/$'\n'}}})
    (( ${#displays[@]} )) && compadd -- "${displays[@]}"
}

_wlout_list_modes() {
    local display_name output
    local -a modes

    # Debug: hardcoded entry to confirm invocation.
    compadd -- "$display_name"

    # Find display name from argv (`words`), handles `--name X`, `--name=X`, `-n X`, `-nX`.
    for (( i=1; i<${#words[@]}; ++i )); do
        case ${words[i]} in
            --name)
                display_name=${words[i+1]}
                ;;
            --name=*)
                display_name=${words[i]#--name=}
                ;;
            -n)
                display_name=${words[i+1]}
                ;;
            -n*)
                display_name=${words[i]#-n}
                ;;
        esac
    done

    [[ -z $display_name ]] && return 0

    output=$(command wlout mode list --name "$display_name" 2>/dev/null) || return 0
    modes=(${(s:\n:)${${output//$'\t'/$'\n'}}})

    (( ${#modes[@]} )) && compadd -- "${modes[@]}"
}
"#,
        );

        script = script.replace(
            "[The name of the display]: :_default",
            "[The name of the display]:display name:_wlout_list_displays",
        );
        script = script.replace(
            "[The mode format is <WIDTH>x<HEIGHT>@<RATE>]: :_default",
            "[The mode format is <WIDTH>x<HEIGHT>@<RATE>]:display mode:_wlout_list_modes",
        );

        buf.write_all(script.as_bytes())
            .expect("failed to write completion script");
    }
}

pub fn completion_command(matches: &ArgMatches, cmd: &mut Command) {
    let shell = matches
        .get_one::<Shell>("shell")
        .copied()
        .expect("Shell argument required");

    eprintln!("Generating completion file for {shell}...");

    if shell != Shell::Zsh {
        generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
        return;
    }

    generate(
        AdvancedZsh {},
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}
