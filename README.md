# csv-translator

Command line tool to transform large survey CSV into translatable YAML file,
and combine them back to a translated CSV file.

## Usage

Firstly, with a raw CSV result sheet, use the following command:
```bash
csv-translator extract original.csv > translation.yaml
```

This will transform a CSV file like:
```csv
Question 1,Question 2,Question 3
Answer B,Answer X,
Answer A,Answer X,Random text
Answer A,Answer Y,
```
into
```yaml
- header: Question 1
  value:
    - Answer A
    - Answer B
- header: Question 2
  value:
    - Answer X
    - Answer Y
- header: Question 3
  value:
    - Random text
```

After you translate the headers and values,
you can run this command:
```bash
csv-translator translate original.csv translation.yaml > translated.csv
```
to combine the translation and the original CSV into a translated one.

In the YAML file, for each column,
in addition to translating values, you can also do:
* Remove the `value` field,
  which means preserve all values as-is.
  This can be useful for columns like serial numbers or E-mail addresses.
* Replace the `value` field with `delete: true`,
  which means the whole column will be deleted from the translated version.

Note that all columns and values are matched in the given order,
so order should not be changed,
and no column or individual value should be removed from the YAML file directly.

## License

Copyright (C) 2019-2020 Xidorn Quan

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
