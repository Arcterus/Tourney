/* Copyright (C) 2013 Arcterus.  All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include <QtGui/QApplication>
#include <QtGui/QWidget>

typedef enum {
	Male = 0,
	Female,
	Other
} Gender;

extern "C" int gui_main(int argc, char *argv[]) {
	QApplication app(argc, argv);

	QWidget window;

	window.resize(250, 150);
	window.setWindowTitle("Tourney");
	window.show();

	return app.exec();
}

#include <stdio.h>
extern "C" void gui_addPlayer(const char *name, Gender gender, int id, int skill, void **team) {
	puts(name);
}

extern "C" void gui_removePlayer(int id) {

}

extern "C" void gui_addWin(int pid, int oppid) {

}
