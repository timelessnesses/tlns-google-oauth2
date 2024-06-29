import json
import jinja2

parsed: dict = json.load(open("./out.json"))

open("sin.rs","w").write(jinja2.FileSystemLoader("./").load(jinja2.Environment(), "scopes.rs.jinja").render(headers=parsed))