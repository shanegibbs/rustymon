# rustymon

* https://stackoverflow.com/questions/19200589/auto-update-tmux-status-bar-with-active-pane-pwd

```
set -g status-left '#(~/bin/display_tmux_pane_pwd.sh)'
set -g status-interval 1
```

```
set -g status-left '#(~/bin/display_tmux_pane_pwd.sh)'
set -g status-left '#(cat ~/.rustymon.status)'
tmux refresh-client -S
```

```
     refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
             [adjustment]
                   (alias: refresh)
             Refresh the current client if bound to a key, or a single
             client if one is given with -t.  If -S is specified, only
             update the client's status line.

             The -U, -D, -L -R, and -c flags allow the visible portion of a
             window which is larger than the client to be changed.  -U moves
             the visible part up by adjustment rows and -D down, -L left by
             adjustment columns and -R right.  -c returns to tracking the
             cursor automatically.  If adjustment is omitted, 1 is used.
             Note that the visible position is a property of the client not
             of the window, changing the current window in the attached ses‐
             sion will reset it.

             -C sets the width and height of a control client and -F sets a
             comma-separated list of flags.  Currently the only flag avail‐
             able is ‘no-output’ to disable receiving pane output.

             -l requests the clipboard from the client using the xterm(1)
             escape sequence and stores it in a new paste buffer.

             -L, -R, -U and -D move the visible portion of the window left,
             right, up or down by adjustment, if the window is larger than
             the client.  -c resets so that the position follows the cursor.
             See the window-size option.

     show-messages [-JT] [-t target-client]
                   (alias: showmsgs)
             Show client messages or server information.  Any messages dis‐
             played on the status line are saved in a per-client message
             log, up to a maximum of the limit set by the message-limit
             server option.  With -t, display the log for target-client.  -J
             and -T show debugging information about jobs and terminals.

     display-time time
             Set the amount of time for which status line messages and other
             on-screen indicators are displayed.  If set to 0, messages and
             indicators are displayed until a key is pressed.  time is in
             milliseconds.

     message-style style
             Set status line message style.  This is used for messages and
             for the command prompt.  For how to specify style, see the
             STYLES section.

     status [off | on | 2 | 3 | 4 | 5]
             Show or hide the status line or specify its size.  Using on
             gives a status line one row in height; 2, 3, 4 or 5 more rows.

     status-format[] format
             Specify the format to be used for each line of the status line.
             The default builds the top status line from the various indi‐
             vidual status options below.

     status-interval interval
             Update the status line every interval seconds.  By default,
             updates will occur every 15 seconds.  A setting of zero dis‐
             ables redrawing at interval.

     status-justify [left | centre | right]
             Set the position of the window list component of the status
             line: left, centre or right justified.

     status-keys [vi | emacs]
             Use vi or emacs-style key bindings in the status line, for
             example at the command prompt.  The default is emacs, unless
             the VISUAL or EDITOR environment variables are set and contain
             the string ‘vi’.

     status-left string
             Display string (by default the session name) to the left of the
             status line.  string will be passed through strftime(3).  Also
             see the FORMATS and STYLES sections.

             For details on how the names and titles can be set see the
             NAMES AND TITLES section.

             Examples are:

                   #(sysctl vm.loadavg)
                   #[fg=yellow,bold]#(apm -l)%%#[default] [#S]

             The default is ‘[#S] ’.

     status-left-length length
             Set the maximum length of the left component of the status
             line.  The default is 10.

     status-left-style style
             Set the style of the left part of the status line.  For how to
             specify style, see the STYLES section.

     status-position [top | bottom]
             Set the position of the status line.

     status-right string
             Display string to the right of the status line.  By default,
             the current pane title in double quotes, the date and the time
             are shown.  As with status-left, string will be passed to
             strftime(3) and character pairs are replaced.

     status-right-length length
             Set the maximum length of the right component of the status
             line.  The default is 40.

     status-right-style style
             Set the style of the right part of the status line.  For how to
             specify style, see the STYLES section.

     status-style style
             Set status line style.  For how to specify style, see the
             STYLES section.


```