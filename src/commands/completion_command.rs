use clap::{ArgMatches, Command};
use clap_complete::{Generator, Shell, generate};
use std::io::{self, Write};

struct AdvancedBash {}

impl Generator for AdvancedBash {
    fn file_name(&self, name: &str) -> String {
        Shell::Bash.file_name(name)
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        let mut temp_buffer = Vec::new();
        generate(
            Shell::Bash,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut temp_buffer,
        );

        let mut script =
            String::from_utf8(temp_buffer).expect("completion script should be valid UTF-8");

        script.push_str(
            r#"
__wlout_list_displays() {
    local display_name output
    local -a displays filtered
    local i j display

    for ((i=1; i<${#COMP_WORDS[@]}; i++)); do
        case "${COMP_WORDS[i]}" in
            info|power|move|mode|mirror)
                for ((j=i+1; j<${#COMP_WORDS[@]}; j++)); do
                    case "${COMP_WORDS[j]}" in
                        above|below|left-of|right-of|position|list|set|current|preferred|help|same-as)
                            break
                            ;;
                        -*)
                            continue
                            ;;
                        *)
                            display_name="${COMP_WORDS[j]}"
                            break
                            ;;
                    esac
                done
                ;;
        esac

        [[ -n "$display_name" ]] && break
    done

    output=$(command wlout list 2>/dev/null) || return 0
    read -a displays <<<"$output"

    if [[ -n "$display_name" ]]; then
        filtered=()
        for display in "${displays[@]}"; do
            [[ "$display" == "$display_name" ]] && continue
            filtered+=("$display")
        done
        displays=("${filtered[@]}")
    fi

    echo "${displays[@]}"
}

__wlout_list_modes() {
    local display_name output
    local -a modes
    local i j

    for ((i=1; i<${#COMP_WORDS[@]}; i++)); do
        if [[ ${COMP_WORDS[i]} == "mode" ]]; then
            for ((j=i+1; j<${#COMP_WORDS[@]}; j++)); do
                case "${COMP_WORDS[j]}" in
                    set|list|current|preferred|help)
                        break
                        ;;
                    -*)
                        continue
                        ;;
                    *)
                        display_name="${COMP_WORDS[j]}"
                        break
                        ;;
                esac
            done
        fi

        [[ -n "$display_name" ]] && break
    done

    [[ -z "$display_name" ]] && return 0

    output=$(command wlout mode "$display_name" list 2>/dev/null) || return 0
    read -a modes <<<"$output"

    echo "${modes[@]}"
}
"#,
        );

        script = script.replace("<display>", "$(__wlout_list_displays)");
        script = script.replace("<other_display>", "$(__wlout_list_displays)");
        script = script.replace("[mode]", "$(__wlout_list_modes)");
        script = script.replace(
            r#"        wlout__move)
            opts="-h --help $(__wlout_list_displays) above below right-of left-of position help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__move)
            local displays="$(__wlout_list_displays)"
            local subs="above below right-of left-of position help"
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "${displays}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=( $(compgen -W "${subs}" -- "${cur}") )
                    ;;
            esac
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__info)
            opts="-h --help $(__wlout_list_displays)"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__info)
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "$(__wlout_list_displays)" -- "${cur}") )
                return 0
            fi
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__power)
            opts="-h --help $(__wlout_list_displays) on off"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__power)
            local displays="$(__wlout_list_displays)"
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "${displays}" -- "${cur}") )
                return 0
            fi
            COMPREPLY=( $(compgen -W "on off" -- "${cur}") )
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__mirror)
            opts="-h --help $(__wlout_list_displays) same-as help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__mirror)
            local displays="$(__wlout_list_displays)"
            local subs="same-as help"
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "${displays}" -- "${cur}") )
                return 0
            fi
            COMPREPLY=( $(compgen -W "${subs}" -- "${cur}") )
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__mirror__same__as)
            opts="-h --help $(__wlout_list_displays)"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__mirror__same__as)
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            COMPREPLY=( $(compgen -W "$(__wlout_list_displays)" -- "${cur}") )
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__mode)
            opts="-h --help $(__wlout_list_displays) list current preferred set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__mode)
            local displays="$(__wlout_list_displays)"
            local subs="list current preferred set help"
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "${displays}" -- "${cur}") )
                return 0
            fi
            COMPREPLY=( $(compgen -W "${subs}" -- "${cur}") )
            return 0
            ;;
"#,
        );
        script = script.replace(
            r#"        wlout__mode__set)
            opts="-h --help $(__wlout_list_modes)"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
"#,
            r#"        wlout__mode__set)
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "-h --help" -- "${cur}") )
                return 0
            fi
            COMPREPLY=( $(compgen -W "$(__wlout_list_modes)" -- "${cur}") )
            return 0
            ;;
"#,
        );

        buf.write_all(script.as_bytes())
            .expect("failed to write completion script");
    }
}

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
    local output display_name
    local -a displays

    # Parse the raw command line instead of `words`, which is rewritten by `_arguments`.
    local -a cmdline
    cmdline=("${(z)BUFFER}")

    # Find the first display argument after a subcommand that expects it.
    for (( i=1; i<=${#cmdline[@]}; ++i )); do
        case ${cmdline[i]} in
            info|power|move|mode|mirror)
                for (( j=i+1; j<=${#cmdline[@]}; ++j )); do
                    case ${cmdline[j]} in
                        # Subcommands that come after the display argument.
                        above|below|left-of|right-of|position|list|set|current|preferred|help)
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
                ;;
        esac

        [[ -n $display_name ]] && break
    done

    output=$(command wlout list 2>/dev/null) || return 0
    # Split on any whitespace (tabs/newlines) to get individual display names.
    displays=(${=output})

    if [[ -n $display_name ]]; then
        # Filter out the display already present on the command line.
        local -a filtered=()
        local display
        for display in "${displays[@]}"; do
            [[ $display == "$display_name" ]] && continue
            filtered+=("$display")
        done
        displays=("${filtered[@]}")
    fi

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
            ":other_display -- Other display:_default",
            ":other_display -- Other display:_wlout_list_displays",
        );
        script = script.replace(
            ":other_display -- The display to mirror:_default",
            ":other_display -- The display to mirror:_wlout_list_displays",
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

    match shell {
        Shell::Zsh => generate(
            AdvancedZsh {},
            cmd,
            cmd.get_name().to_string(),
            &mut io::stdout(),
        ),
        Shell::Bash => generate(
            AdvancedBash {},
            cmd,
            cmd.get_name().to_string(),
            &mut io::stdout(),
        ),
        _ => generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout()),
    };
}
