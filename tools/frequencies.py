#!/bin/python

   #
   # jevlor - extensible random word generator
   #
   # Copyright (C) 2026  Fedor Veselov
   #
   # This program is free software: you can redistribute it and/or modify
   # it under the terms of the GNU General Public License as published by
   # the Free Software Foundation, either version 3 of the License, or
   # (at your option) any later version.
   #
   # This program is distributed in the hope that it will be useful,
   # but WITHOUT ANY WARRANTY; without even the implied warranty of
   # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   # GNU General Public License for more details.
   #
   # You should have received a copy of the GNU General Public License
   # along with this program.  If not, see <https://www.gnu.org/licenses/>.
   #
import pandas as pd

letter_pairs = []
letter_counts = {}

forbidden = input('List of forbidden characters ("-" by default) ')
if forbidden == '':
    forbidden = "-"

while True:
    try: 
        path = input('Path to .csv dataset ')
        delim = input('Delimiter of .csv file (default: ";") ')
        if delim == '':
            delim = ';'

        df = pd.read_csv(path, delimiter=delim)
        target = input(f'Target column (avaiable: {df.columns.to_list}) ')
        for word in df[target]:
            for ch in word.lower():
                if ch in forbidden or ch == '':
                    continue
                if not ch in letter_counts:
                    letter_counts[ch] = 0
                else:
                    letter_counts[ch] += 1
            for i in range(len(word)-1):
                letter_pairs.append(word[i]+word[i+1])
    except EOFError:
        break
    except FileNotFoundError:
        print('Try again')

# Counting letter weights and calculating ranks
total_letters = 0
ranks = list(letter_counts.keys())
for k,v in letter_counts.items():
    total_letters += v
    ranks.append(k)

per_letter_probabilities = {}
for k,v in letter_counts.items():
    per_letter_probabilities[k] = v / total_letters

ranks.sort(key=lambda x: letter_counts[x])

write_path = input('Output path (default: results/frequencies.txt ')
if write_path == '':
    write_path = "results/frequencies.txt"

with open(write_path, "w") as f:
    f.write(f'ranks={ranks}\n')
    f.write(f'weights={per_letter_probabilities}\n')

