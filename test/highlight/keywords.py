if foo():
# <- keyword
    pass
    # <- keyword
elif bar():
# <- keyword
    pass
else:
# <- keyword
    foo

return
# ^ keyword
raise e
# ^ keyword

for i in foo():
# <- keyword
#   ^ variable
#     ^ keyword
#        ^ function
    continue
    # <- keyword
    break
    # <- keyword

a and b or c
# ^ keyword
#     ^ variable
#       ^ keyword
