## Language Grammar
```
{FILE} -> {LINE}\n | {LINE} | :empty:
{LINE} -> {PREFIX_COMMAND} | {INFIX_COMMAND}
{PREFIX_COMMAND} -> {COMMAND} {VARIABLE} {PARAMETERS}
{INFIX_COMMAND} -> {VARIABLE} {COMMAND} {PARAMETERS}
{COMMAND} -> img | layer | save
{VARIABLE} -> [a-z0-9]* - {COMMAND} # words made of letters and numbers excluding COMMANDS
{PARAMETERS} -> {VALUE} {PARAMETERS} | {VALUE}
{VALUE} -> :NUMBER: | ':STRING:'
```
