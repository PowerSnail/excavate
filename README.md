# excavate

A command line tool to excavate fields from multi-column texts.
A command line tool to excavate fields from multi-column texts.

## Usage:

```bash
excavate <FIELDS>
```

<FIELDS> In the format of comma delimited positive integers. At least one field must be specified. The field number starts from 0.

For example:

Printing the all the process IDs matching "bash":

```bash
ps aux | grep bash | excavate 1
```
