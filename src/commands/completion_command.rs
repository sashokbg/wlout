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
    # Split on any whitespace (tabs/newlines) to get individual display names.
    displays=(${=output})
    (( ${#displays[@]} )) && compadd -- "${displays[@]}"
}

_wlout_list_modes() {
    local display_name output
    local -a modes

    # Parse the raw command line instead of `words`, which is rewritten by `_arguments`.
    local -a cmdline
    cmdline=("${(z)BUFFER}")

    # CLI shape is `wlout mode <display> set <mode>`.
    # Grab the first non-flag argument after `mode` and before the subcommand.
    for (( i=1; i<=${#cmdline[@]}; ++i )); do
        if [[ ${cmdline[i]} == "mode" ]]; then
            for (( j=i+1; j<=${#cmdline[@]}; ++j )); do
                case ${cmdline[j]} in
                    set|list|current|help)
                        break
                        ;;
                    -*)
                        continue
                        ;;
                    *)
                        display_name=${cmdline[j]}
                        break
                        ;;
                esac
            done
        fi

        [[ -n $display_name ]] && break
    done

    [[ -z $display_name ]] && return 0

    output=$(command wlout mode "$display_name" list 2>/dev/null) || return 0
    # Modes are whitespace separated, split them and add as completions.
    modes=(${=output})

    (( ${#modes[@]} )) && compadd -- "${modes[@]}"
}
"#,
        );

        script = script.replace(
            ":display -- The name of the display:_default",
            ":display -- The name of the display:_wlout_list_displays",
        );
        script = script.replace(
            "::mode -- The mode format is <WIDTH>x<HEIGHT>@<RATE>:_default",
            "::mode -- The mode format is <WIDTH>x<HEIGHT>@<RATE>:_wlout_list_modes",
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
