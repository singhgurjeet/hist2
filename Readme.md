hist2 is a command line program making simple histograms on the command line.

### Quick demo

Let's say you're playing with the data from the [Data Science Toolkit](https://github.com/petewarden/dstkdata), 
which contains several CSV files. Maybe you're interested in the distribution of the latitudes of 
all the cities in the world:

```bash
$ curl -LO https://burntsushi.net/stuff/worldcitiespop.csv
$ cat worldcitiespop.csv | cut -f6 -d, | hist2
```
![Demo](https://drive.google.com/open?id=1pi7_66ko1sQnoGJe348XnkKXV4ur88dZ)