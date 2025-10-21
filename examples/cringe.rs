use hyprland::dispatch::*;


fn main() -> hyprland::Result<()> {
    // let line:&str = "[monitor 0;workspace special silent;move 1814 757;size 1352 663;fullscreenstate 0] /usr/lib/vesktop/vesktop";
    // let line:&str = "hyprctl dispatch exec \"[workspace special] vesktop\"";
    // let line:&str = "[movetoworkspacesilent special] /usr/lib/vesktop/vesktop";
    // let line:&str = "/usr/lib/vesktop/vesktop && sleep 500 && movetoworkspacesilent special";
    // let line:&str = "movetoworkspacesilent special, kitty";

    // let line:&str = "[workspace special] /usr/lib/vesktop/vesktop"; // OISDFKJDHS WORKSX but not for vesktop
    let line:&str = "[monitor 0;workspace special silent;move 14 59;size 1931 1367;fullscreenstate 0] firefox"; // works for firefox
    let _ = hyprland::dispatch!(Exec, &line);
    // let _ = hyprland::dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Special(None), None);
    // println!("{}", cringe);
    Ok(())
}