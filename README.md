# Usage
```
tcli 0.1
control tauon music box from the command line

USAGE:
    tcli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
        --next          Next song in playlist
        --pause         Pause playback
        --play          Start playback
        --play-pause    Switches play to pause and vice versa
        --prev          Previous song in playlist
        --stop          Stop playback
    -V, --version       Prints version information

OPTIONS:
        --change-volume <changevolume>    Change volume by x units, anything in between -100 and 100
        --seek <seek>                     Seeks x milliseconds ahead or behind
        --set-volume <setvolume>          Set volume to x units, anything in between 0 and 100
        --status <status>
            Prints current player status in customizable format
                %A = Album
                %T = Track Name
                %N = Track Number
                %R = Artist
                %P = Song Progress (in seconds)
                %D = Song Duration (in seconds)
             [default: %R - %A - %T]

        --url <url>                       api url [default: http://localhost:7814/api1]
```
