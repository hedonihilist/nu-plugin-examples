# Generate data in nushell

this code shows how to generate data in plugin.

### Run
This plugin will generate a row of data comprised of the parameters passed in.
```
> print_args what ever you pass in
───┬────────┬────────┬────────┬────────┬────────
 # │ arg[0] │ arg[1] │ arg[2] │ arg[3] │ arg[4] 
───┼────────┼────────┼────────┼────────┼────────
 0 │ what   │ ever   │ you    │ pass   │ in     
───┴────────┴────────┴────────┴────────┴────────
```
