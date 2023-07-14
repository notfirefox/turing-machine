#include "impl.h"

bool accept1(const int q) { return q == 3; }

struct delta_result delta1(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == '0' || param.input == '1') {
			result.state = 0;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 1;
			result.output = BLANK;
			result.move = left;
		}
	} else if (param.state == 1) {
		if (param.input == '1') {
			result.state = 1;
			result.output = '0';
			result.move = left;
		} else if (param.input == '0' || param.input == BLANK) {
			result.state = 2;
			result.output = '1';
			result.move = left;
		}
	} else if (param.state == 2) {
		if (param.input == '0' || param.input == '1') {
			result.state = 2;
			result.output = param.input;
			result.move = left;
		} else if (param.input == BLANK) {
			result.state = 3;
			result.output = BLANK;
			result.move = right;
		}
	}

	return result;
}

bool accept2(const int q) { return q == 3; }

struct delta_result delta2(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == '0') {
			result.state = 1;
			result.output = '1';
			result.move = right;
		}
	} else if (param.state == 1) {
		if (param.input == '1') {
			result.state = 2;
			result.output = '0';
			result.move = left;
		} else if (param.input == BLANK) {
			result.state = 3;
			result.output = BLANK;
			result.move = right;
		}
	} else if (param.input == 2) {
		if (param.input == '1') {
			result.state = 0;
			result.output = '1';
			result.move = right;
		}
	}

	return result;
}

bool accept3(const int q) { return q == 2; }

struct delta_result delta3(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == 'a') {
			result.state = 0;
			result.output = 'b';
			result.move = right;
		} else if (param.input == 'b') {
			result.state = 0;
			result.output = 'a';
			result.move = right;
		} else if (param.input == 'c' || param.input == 'd') {
			result.state = 0;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 1;
			result.output = BLANK;
			result.move = left;
		}
	} else if (param.state == 1) {
		if (param.input == 'a' || param.input == 'b' ||
		    param.input == 'c' || param.input == 'd') {
			result.state = 1;
			result.output = param.input;
			result.move = left;
		} else if (param.input == BLANK) {
			result.state = 2;
			result.output = BLANK;
			result.move = right;
		}
	}

	return result;
}

bool accept4(const int q) { return q == 8; }

struct delta_result delta4(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == 'a') {
			result.state = 0;
			result.output = 'A';
			result.move = right;
		} else if (param.input == 'b') {
			result.state = 0;
			result.output = 'B';
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 1;
			result.output = BLANK;
			result.move = left;
		}
	} else if (param.state == 1) {
		if (param.input == 'a' || param.input == 'b' ||
		    param.input == 'A' || param.input == 'B') {
			result.state = 1;
			result.output = param.input;
			result.move = left;
		} else if (param.input == BLANK) {
			result.state = 2; // state = 2 => found first A or B
			result.output = BLANK;
			result.move = right;
		}
	} else if (param.state == 2) {
		if (param.input == 'A') {
			result.state = 3; // state = 3 => append a
			result.output = 'a';
			result.move = right;
		} else if (param.input == 'B') {
			result.state = 4; // state 4 => append b
			result.output = 'b';
			result.move = right;
		}
	} else if (param.state == 3) {
		if (param.input == 'a' || param.input == 'b' ||
		    param.input == 'A' || param.input == 'B') {
			result.state = param.state;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 5; // state = 5 => go left to next blank
			result.output = 'a';
			result.move = left;
		}
	} else if (param.state == 4) {
		if (param.input == 'a' || param.input == 'b' ||
		    param.input == 'A' || param.input == 'B') {
			result.state = param.state;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 5; // state = 5 => go left to next blank
			result.output = 'b';
			result.move = left;
		}
	} else if (param.state == 5) {
		if (param.input == 'a' || param.input == 'b' ||
		    param.input == 'A' || param.input == 'B') {
			result.state = 5;
			result.output = param.input;
			result.move = left;
		} else if (param.input == BLANK) {
			result.state =
			    6; // state = 6 => reached first character
			result.output = BLANK;
			result.move = right;
		}
	} else if (param.state == 6) {
		if (param.input == 'a' || param.input == 'b') {
			result.state = 6; // still searching for A or B
			result.output = param.input;
			result.move = right;
		} else if (param.input == 'A' || param.input == 'B') {
			result.state = 2; // found A or B
			result.output = param.input;
			result.move = none;
		} else if (param.input == BLANK) {
			result.state = 7; // reached end without finding A or B
			result.output = BLANK;
			result.move = left;
		}
	} else if (param.state == 7) {
		if (param.input == 'a' || param.input == 'b') {
			result.state = 7; // did not find leftmost BLANK yet
			result.output = param.input;
			result.move = left;
		} else if (param.input == BLANK) {
			result.state = 8; // found beginning of word
			result.output = BLANK;
			result.move = right;
		}
	}

	return result;
}

bool accept5(const int q) { return q == 5; }

struct delta_result delta5(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == 'a') {
			result.state = 1;
			result.output = 'A';
			result.move = right;
		}
	} else if (param.state == 1) {
		if (param.input == 'X') {
			result.state = 1;
			result.output = 'X';
			result.move = right;
		} else if (param.input == 'a') {
			result.state = 2;
			result.output = 'X';
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 5;
			result.output = BLANK;
			result.move = none;
		}
	} else if (param.state == 2) {
		if (param.input == 'X') {
			result.state = 2;
			result.output = 'X';
			result.move = right;
		} else if (param.input == 'a') {
			result.state = 3;
			result.output = 'a';
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 4;
			result.output = BLANK;
			result.move = left;
		}
	} else if (param.state == 3) {
		if (param.input == 'a') {
			result.state = 2;
			result.output = 'X';
			result.move = right;
		} else if (param.input == 'X') {
			result.state = 3;
			result.output = 'X';
			result.move = right;
		}
	} else if (param.state == 4) {
		if (param.input == 'X' || param.input == 'a') {
			result.state = 4;
			result.output = param.input;
			result.move = left;
		} else if (param.input == 'A') {
			result.state = 1;
			result.output = 'A';
			result.move = right;
		}
	}

	return result;
}

bool accept6(const int q) { return q == 4; }

struct delta_result delta6(struct delta_param param) {
	struct delta_result result;
	result.state = -1;
	result.output = 0;
	result.move = none;

	if (param.state == 0) {
		if (param.input == 'a') {
			result.state = 1;
			result.output = 'A';
			result.move = right;
		} else if (param.input == 'b') {
			result.state = 2;
			result.output = 'B';
			result.move = right;
		}
	} else if (param.state == 1) {
		if (param.input == 'a' || param.input == 'b') {
			result.state = 1;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 3;
			result.output = 'a';
			result.move = left;
		}
	} else if (param.state == 2) {
		if (param.input == 'a' || param.input == 'b') {
			result.state = 2;
			result.output = param.input;
			result.move = right;
		} else if (param.input == BLANK) {
			result.state = 3;
			result.output = 'b';
			result.move = left;
		}
	} else if (param.state == 3) {
		if (param.input == BLANK) {
			result.state = 4;
			result.output = BLANK;
			result.move = right;
		} else if (param.input == 'A') {
			result.state = 0;
			result.output = 'a';
			result.move = right;
		} else if (param.input == 'B') {
			result.state = 0;
			result.output = 'b';
			result.move = right;
		}
	}

	return result;
}
