# Titulus

>from Latin, literally: inscription, label, **title**.

Titulus is a minimal browser homepage, heavily inspired by [tilde](https://github.com/xvvvyz/tilde/tree/main).

https://github.com/JohnBCoding/titulus/assets/12802117/b9ab5192-8085-4d01-86a1-b4b46908ca5c

## Configure

To configure the homepage, click the settings button or hit ESC to bring up the settings view.

On the settings menu you can set the name, url and hotkey for each slot on the homepage.

You can also change the default search option in the settings. Be sure to put `{}` wherever the search value should be inserted.

## Usage

**To go to a site**, type the corresponding hotkey and press <kbd>Enter</kbd>.

e.g. if you have GitHub assigned to `g` then `g` + <kbd>Enter</kbd> will redirect you to
GitHub

**To search a site**, you must first set the search URL in the settings. then you can type a space after the hotkey followed by your
search.

e.g. if you have Reddit assigned to `r` then `r programming` <kbd>Enter</kbd>
[Will search reddit for programming](https://www.reddit.com/search/?q=programming)

**To go to a specific path on a site**, type the path after the hotkey

e.g. if you have Reddit assigned to `r` then `r/r/programming` <kbd>Enter</kbd> will redirect you to
[reddit.com/r/programming](https://www.reddit.com/r/programming)

**If your input doesn't match a key**, a search will be triggered.

e.g. `google` <kbd>Enter</kbd> will search the configured search url for google **(Default is DuckDuckGo)**

Auto complete results will also be displayed and can be cycled through with <kbd>Up</kbd> / <kbd>Down</kbd>

**To access a site directly**, enter the URL and hit <kbd>Enter</kbd>

e.g. `example.com` <kbd>Enter</kbd> will redirect you to [example.com](exmaple.com)

## Access

I have a version hosted via GitHub pages at https://www.titulus.me/

I also have a [Docker Image](https://hub.docker.com/repository/docker/johnbcoding/titulus/general) available or you can build and run it yourself locally.
