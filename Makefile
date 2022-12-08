K=kernel
U=user

mkfs/mkfs: mkfs/mkfs.c mkfs/fs.h mkfs/param.h
	gcc -Werror -Wall -I. -o mkfs/mkfs mkfs/mkfs.c

# Prevent deletion of intermediate files, e.g. cat.o, after first build, so
# that disk image changes after first build are persistent until clean.  More
# details:
# http://www.gnu.org/software/make/manual/html_node/Chained-Rules.html
.PRECIOUS: %.o

UPROGS =

fs.img: mkfs/mkfs README.md $(UPROGS)
	mkfs/mkfs fs.img README.md $(UPROGS)
