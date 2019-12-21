# csv-translator

Command line tool to transform large survey CSV into translatable YAML file,
and combine them back to a translated CSV file.

Usage:
```bash
csv-translator extract original.csv > translation.yaml
# Translate translation.yaml...
csv-translator translate original.csv translation.yaml > translated.csv
```

## License

Copyright (C) 2019 Xidorn Quan

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
