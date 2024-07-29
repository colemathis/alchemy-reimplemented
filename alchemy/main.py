import alchemy

with open("fontana_res.txt", "r") as f:
    expressions = f.readlines()

new_expression = alchemy.run_alchemy(expressions, "default/default_config.json")

print(new_expression)