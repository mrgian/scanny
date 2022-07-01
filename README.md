# Scanny

Simple domain scanner wirtten in Rust

## Usage
```
scanny <domain.com>
```

Or if you want compile from source, install Rust (don't need to tell you how to do this), and then:
```
git clone https://github.com/mrgian/scanny.git
cd scanny
cargo run -- <domain.com>
```
## Example
```
scanny univpm.it
```

and you'll get:

```
aule.univpm.it
prodapps.econ.univpm.it
www.cad.univpm.it
iris.univpm.it
psa.univpm.it
iplanta.univpm.it
yourfuturefestival.univpm.it
gretlml.univpm.it
afp.univpm.it
survey.univpm.it
www.medicinadellavoro.univpm.it
foritaal2013.univpm.it
...
```

## To do
Scanny is only able to scan for subdomains at the moment, but I will add many more features in the future, such as:

- Port scanner (coming very soon)
- Vulnerability scanner
- And many more
