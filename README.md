# splitv

Show multiple panes in the terminal.  Use this to show diff.

In the below example, I specified three panes of width 3, 30, and 40
respectively.  Each line is wrapped at the specified length and the line of the
same line number is aligned in all panes.

```
1   | Lorem ipsum dolor sit amet, co | Lorem ipsum dolor sit amet, consectetur
    | nsectetur adipiscing elit,     | adipiscing elit,
2   | sed do eiusmod tempor incididu | sed do eiusmod tempor incididunt ut labo
    | nt ut labore et dolore magna a | re et dolore magna aliqua. Ut enim ad mi
    | liqua. Ut enim ad minim veniam | nim veniam, quis nostrud exercitation ul
    | , quis nostrud exercitation ul | lamco laboris nisi ut aliquip ex ea comm
    | lamco laboris nisi ut aliquip  | odo consequat.
    | ex ea commodo consequat.       |
3   | Duis aute irure dolor in repre | Duis aute irure dolor in reprehenderit i
    | henderit in voluptate velit es | n voluptate velit esse cillum dolore eu
    | se cillum dolore eu fugiat nul | fugiat nulla pariatur.
    | la pariatur.                   |
4   | Excepteur sint occaecat cupida | とりあえずここに自然に日本語がまぎれてき
    | tat non proident, sunt in culp | てもたぶんいい感じに切ってくれるはずだよ
    | a qui officia deserunt mollit  | ね
    | anim id est laborum.           |
```
