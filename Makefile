BINLOCALDIR := ./target/debug/BigLet
PREFIX := /usr/local
BINDIR=${PREFIX}/bin/
BINNAME := biglet
DOCDIR=${PREFIX}/share/doc/BigLet
MANDIR=${PREFIX}/share/man
LICENSEDIR=${PREFIX}/share/licenses/BigLet
INSTALLDIR=${BINDIR}${BINNAME}
rm = rm -rf

.PHONY: install
install:
	cargo build
	cp -r ${BINLOCALDIR} ${INSTALLDIR}
	@echo "Done Installing biglet"

.PHONY: update
update:
	git pull origin master --rebase
	cargo build
	cp -r ${BINLOCALDIR} ${INSTALLDIR}
	@echo "Done Updating biglet"

.PHONY: uninstall
uninstall:
	@$(rm) ${INSTALLDIR}
	@echo "Done deleting biglet"

.PHONY: doc
doc:
	mkdir -p ${MANDIR}/man1
	mkdir -p ${DESTDIR}${DOCDIR}
	mkdir -p ${DESTDIR}${LICENSEDIR}
	chmod 644 ./docs/man/BigLet.1 ./LICENSE.md
	cp -r ./docs/man/BigLet.1 ${DESTDIR}${MANDIR}/man1
	cp -r ./LICENSE.md ${DESTDIR}${LICENSEDIR}
