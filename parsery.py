import yarl
import json

content = open("./info.txt")
unparsable_line = open("./unparsable.txt", "w+")

API = str
DOCUMENTATION = str
NAME = str

infos: dict[str, list[tuple[API, DOCUMENTATION, NAME]]] = {}
current_api = ""
for l in content.readlines():
    l = l.strip("\n")
    if l == "Scopes":
        continue
    elif l.startswith("http"):
        # print(l)
        try:
            try:
                (http_api, doc) = l.split("\t")
            except:
                (http_api, doc) = l.split(" ")
        except:
            unparsable_line.write(l + "\n")
        http_api = yarl.URL(http_api)
        name = http_api.parts[-2] + " " + http_api.parts[-1]
        name = "".join(" ".join((" ".join(name.split(".")).split("-"))).title().split(" ")).replace("only", "Only").strip("(").strip(")")
        try:
            infos[current_api].append((str(http_api), doc, name))
        except KeyError:
            infos[current_api] = [(str(http_api), doc, name)]
        print(http_api, doc, name)
        continue
    elif "API" in l:
        current_api = "".join("".join(l.split(",")).split(" "))
        current_api = current_api.replace("&","And").replace(".", "Point")
        continue
    unparsable_line.write(l+ "\n")

json.dump(infos, open("./out.json", "w"), sort_keys=True, indent=4)