Processed a 1.2GB File in 16.7s, could be faster, could be way slower.

# Installation

```bash
git clone https://github.com/gbrls/masscan-xml-parser
cd masscan-xml-parser
cargo install --path .
```

# Usage 

```bash
$ masscan-xml-parser a.xml
1672681288 0.0.0.0 ipv4 tcp 443 open syn-ack 56
1672699992 0.0.0.1 ipv4 tcp 443 open syn-ack 235
1672698324 0.0.0.2 ipv4 tcp 8080 open syn-ack 49
1672705424 0.0.0.3 ipv4 tcp 80 open syn-ack 51
1672703900 0.0.0.4 ipv4 tcp 443 open syn-ack 58
$ masscan-xml-parser a.xml |  awk '{print $2}'
0.0.0.0
0.0.0.1
0.0.0.2
0.0.0.3
0.0.0.4
```

Sample input:
```xml
<host endtime="1672681288"><address addr="0.0.0.0" addrtype="ipv4"/><ports><port protocol="tcp" portid="443"><state state="open" reason="syn-ack" reason_ttl="56"/></port></ports></host>
<host endtime="1672699992"><address addr="0.0.0.1" addrtype="ipv4"/><ports><port protocol="tcp" portid="443"><state state="open" reason="syn-ack" reason_ttl="235"/></port></ports></host>
<host endtime="1672698324"><address addr="0.0.0.2" addrtype="ipv4"/><ports><port protocol="tcp" portid="8080"><state state="open" reason="syn-ack" reason_ttl="49"/></port></ports></host>
<host endtime="1672705424"><address addr="0.0.0.3" addrtype="ipv4"/><ports><port protocol="tcp" portid="80"><state state="open" reason="syn-ack" reason_ttl="51"/></port></ports></host>
<host endtime="1672703900"><address addr="0.0.0.4" addrtype="ipv4"/><ports><port protocol="tcp" portid="443"><state state="open" reason="syn-ack" reason_ttl="58"/></port></ports></host>
```

Sample output:

```
1672681288 0.0.0.0 ipv4 tcp 443 open syn-ack 56 
1672699992 0.0.0.1 ipv4 tcp 443 open syn-ack 235 
1672698324 0.0.0.2 ipv4 tcp 8080 open syn-ack 49 
1672705424 0.0.0.3 ipv4 tcp 80 open syn-ack 51 
1672703900 0.0.0.4 ipv4 tcp 443 open syn-ack 58 
```
