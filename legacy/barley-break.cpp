#include <fstream>
#include <cstdlib>
#include <cstdio>
#include "conio.h"
#include <ctime>
#include <Windows.h>
#include <queue>
#include <vector>

using namespace std;

const int N = 4;

struct coord
{
	int x, y;
};

bool equal(coord a, coord b)
{
	return a.x == b.x && a.y == b.y;
}

int field[N][N] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0 };
int cor[16] = { 1, 2, 3, 4, 5, 9, 13, 6, 7, 8, 10, 14, 11, 12, 15, 0 };
bool isCor[16];
coord corFlw[16];
coord flw[16];
int cnt = 0;
int record;

//-----------------Inizialize location of numbers and correct location of numbers

void iniz()
{
	for (int i = 0; i < N; i++)
	{
		for (int j = 0; j < N; j++)
		{
			flw[field[i][j]].x = j;
			flw[field[i][j]].y = i;
		}
	}
	for (int i = 0; i < 16; i++)
		isCor[i] = false;
	for (int i = 0; i < N; i++)
	{
		for (int j = 0; j < N; j++)
		{
			if (i != 3 || j != 3)
			{
				corFlw[i * 4 + j + 1].x = j;
				corFlw[i * 4 + j + 1].y = i;
			}
		}
	}
	corFlw[0].x = 3;
	corFlw[0].y = 3;
}

//-----------------End inizialize

//-----------------Work with record file

void readRecord()
{
	ifstream cin("record.txt");
	cin >> record;
}

void printRecord()
{
	ofstream cout("record.txt");
	cout << record;
}

//-----------------End work with record file

//-----------------Print functions

void print()
{
	system("cls");
	printf("\n");
	for (int i = 0; i < N; i++)
	{
		printf("   ");
		for (int j = 0; j < N; j++)
		{
			if (field[i][j] == 0) printf("%2c ", ' ');
			else printf("%2d ", field[i][j]);
		}
		if (i == 1)
		{
			printf("            Your score: %d", cnt);
		}
		printf("\n");
	}
	printf("\n\n Esc - exit, I - solve this puzzle, UP DOWN LEFT RIGHT - control the game, R - restart");
}

void printWin()
{
	system("cls");
	if (record > cnt)
	{
		printf("\n\n\n   You have solved this puzzle in %d moves, and you have beated the record!!!\n\n\n   Esc - exit, R - restart", cnt);
		record = cnt;
		printRecord();
	}
	else printf("\n\n\n   You have solved this puzzle in %d moves, but your record is %d moves.\n\n\n   Esc - exit, R - restart", cnt, record);
}

//-----------------End print functions

//-----------------Steps to transport empty cell

void trans(int a, int b)
{
	cnt++;
	int num = field[flw[0].y + b][flw[0].x + a];
	swap(field[flw[0].y][flw[0].x], field[flw[0].y + b][flw[0].x + a]);
	swap(flw[0], flw[num]);
}

void step(int a, bool game)
{
	if (a == 1 && flw[0].x != N - 1)
	{
		trans(1, 0);
	}
	else if (a == 2 && flw[0].y != N - 1)
	{
		trans(0, 1);
	}
	else if (a == 3 && flw[0].x != 0)
	{
		trans(-1, 0);
	}
	else if (a == 4 && flw[0].y != 0)
	{
		trans(0, -1);
	}
	if (game)
	{
		Sleep(100);
		print();
	}
}

//-----------------End steps

//-----------------Random inizialize field

void random()
{
	srand(time(0));
	for (int i = 0; i < 100000; i++)
	{
		int a = rand() % 4;
		a++;
		step(a, 0);
	}
}

//-----------------End random

//-----------------Check correct location of numbers

bool check()
{
	for (int i = 1; i <= 15; i++)
	{
		if (!equal(flw[i], corFlw[i]))
			return false;
	}
	return true;
}

//-----------------End check

//-----------------Solve Barley-Break

void bfs(int num, coord a, int x, int y)
{
	queue <coord> q;
	q.push(flw[num]);
	int used[4][4] = { 0 };
	used[flw[num].y][flw[num].x] = 1;
	bool w[4][4] = { 0 };
	w[flw[num].y][flw[num].x] = 1;
	w[a.y + y][a.x + x] = 1;
	while (!q.empty() && !equal(a, q.front()))
	{
		if (q.front().x != 0 && !isCor[field[q.front().y][q.front().x - 1]] && !w[q.front().y][q.front().x - 1])
		{
			q.push(flw[field[q.front().y][q.front().x - 1]]);
			used[q.front().y][q.front().x - 1] = used[q.front().y][q.front().x] + 1;
			w[q.front().y][q.front().x - 1] = true;
		}

		if (q.front().x != N - 1 && !isCor[field[q.front().y][q.front().x + 1]] && !w[q.front().y][q.front().x + 1])
		{
			q.push(flw[field[q.front().y][q.front().x + 1]]);
			used[q.front().y][q.front().x + 1] = used[q.front().y][q.front().x] + 1;
			w[q.front().y][q.front().x + 1] = true;
		}

		if (q.front().y != 0 && !isCor[field[q.front().y - 1][q.front().x]] && !w[q.front().y - 1][q.front().x])
		{
			q.push(flw[field[q.front().y - 1][q.front().x]]);
			used[q.front().y - 1][q.front().x] = used[q.front().y][q.front().x] + 1;
			w[q.front().y - 1][q.front().x] = true;
		}

		if (q.front().y != N - 1 && !isCor[field[q.front().y + 1][q.front().x]] && !w[q.front().y + 1][q.front().x])
		{
			q.push(flw[field[q.front().y + 1][q.front().x]]);
			used[q.front().y + 1][q.front().x] = used[q.front().y][q.front().x] + 1;
			w[q.front().y + 1][q.front().x] = true;
		}
		q.pop();
	}
	if (!q.empty())
	{
		vector <int> way;
		coord c = a;
		while (!q.empty() && !equal(c, flw[num]))
		{
			if (c.x != 0 && used[c.y][c.x - 1] == used[c.y][c.x] - 1)
			{
				way.push_back(1);
				c.x--;
			}
			else if (c.x != N - 1 && used[c.y][c.x + 1] == used[c.y][c.x] - 1)
			{
				way.push_back(3);
				c.x++;
			}
			else if (c.y != 0 && used[c.y - 1][c.x] == used[c.y][c.x] - 1)
			{
				way.push_back(2);
				c.y--;
			}
			else if (c.y != N - 1 && used[c.y + 1][c.x] == used[c.y][c.x] - 1)
			{
				way.push_back(4);
				c.y++;
			}
		}
		for (int i = way.size() - 1; i >= 0; i--)
		{
			step(way[i], 1);
		}
	}
}

bool can(int ind, int turn)
{
	if (turn == 1)
		return flw[0].x == flw[ind].x - 1 && flw[0].y == flw[ind].y;
	if (turn == 2)
		return flw[0].y == flw[ind].y - 1 && flw[0].x == flw[ind].x;
	if (turn == 3)
		return flw[0].x == flw[ind].x + 1 && flw[0].y == flw[ind].y;
	if (turn == 4)
		return flw[0].y == flw[ind].y + 1 && flw[0].x == flw[ind].x;
	return false;
}

void transport(int ind, int turn)
{
	coord a = flw[ind];
	if (turn == 1)
	{
		a.x--;
		bfs(0, a, 1, 0);
	}
	else if (turn == 2)
	{
		a.y--;
		bfs(0, a, 0, 1);
	}
	else if (turn == 3)
	{
		a.x++;
		bfs(0, a, -1, 0);
	}
	else if (turn == 4)
	{
		a.y++;
		bfs(0, a, 0, -1);
	}
	if (can(ind, turn))
		step(turn, 1);
}

int maxY = 0, maxX = 0;

void bfs2(int num, coord a)
{
	queue <coord> q;
	q.push(flw[num]);
	int used[4][4] = { 0 };
	used[flw[num].y][flw[num].x] = 1;
	bool w[4][4] = { 0 };
	w[flw[num].y][flw[num].x] = 1;
	while (!q.empty() && !equal(a, q.front()))
	{
		if (q.front().x != 0 && !isCor[field[q.front().y][q.front().x - 1]] && !w[q.front().y][q.front().x - 1])
		{
			q.push(flw[field[q.front().y][q.front().x - 1]]);
			used[q.front().y][q.front().x - 1] = used[q.front().y][q.front().x] + 1;
			w[q.front().y][q.front().x - 1] = true;
		}

		if (q.front().x != N - 1 && !isCor[field[q.front().y][q.front().x + 1]] && !w[q.front().y][q.front().x + 1])
		{
			q.push(flw[field[q.front().y][q.front().x + 1]]);
			used[q.front().y][q.front().x + 1] = used[q.front().y][q.front().x] + 1;
			w[q.front().y][q.front().x + 1] = true;
		}

		if (q.front().y != 0 && !isCor[field[q.front().y - 1][q.front().x]] && !w[q.front().y - 1][q.front().x])
		{
			q.push(flw[field[q.front().y - 1][q.front().x]]);
			used[q.front().y - 1][q.front().x] = used[q.front().y][q.front().x] + 1;
			w[q.front().y - 1][q.front().x] = true;
		}

		if (q.front().y != N - 1 && !isCor[field[q.front().y + 1][q.front().x]] && !w[q.front().y + 1][q.front().x])
		{
			q.push(flw[field[q.front().y + 1][q.front().x]]);
			used[q.front().y + 1][q.front().x] = used[q.front().y][q.front().x] + 1;
			w[q.front().y + 1][q.front().x] = true;
		}
		q.pop();
	}
	if (!q.empty())
	{
		vector <int> way;
		coord c = a;
		while (!equal(c, flw[num]))
		{
			if (c.x != 0 && used[c.y][c.x - 1] == used[c.y][c.x] - 1)
			{
				way.push_back(3);
				c.x--;
			}
			else if (c.x != N - 1 && used[c.y][c.x + 1] == used[c.y][c.x] - 1)
			{
				way.push_back(1);
				c.x++;
			}
			else if (c.y != 0 && used[c.y - 1][c.x] == used[c.y][c.x] - 1)
			{
				way.push_back(4);
				c.y--;
			}
			else if (c.y != N - 1 && used[c.y + 1][c.x] == used[c.y][c.x] - 1)
			{
				way.push_back(2);
				c.y++;
			}
		}
		for (int i = way.size() - 1; i >= 0; i--)
		{
			transport(num, way[i]);
		}
	}
}

void move(int ind)
{
	coord a = corFlw[ind];
	if (ind == 4 || ind == 8)
	{
		bfs2(ind, a);
		if (!equal(corFlw[ind], flw[ind]))
		{
			a.y += 2;
			bfs2(ind, a);
			a.y--;
			bfs(0, a, 0, 1);
			step(4, 1);
			step(3, 1);
			step(2, 1);
			step(1, 1);
			step(2, 1);
			step(3, 1);
			step(4, 1);
			step(4, 1);
			step(1, 1);
			step(2, 1);
		}
	}
	else if (ind == 13 || ind == 14)
	{
		bfs2(ind, a);
		if (!equal(corFlw[ind], flw[ind]))
		{
			a.x += 2;
			bfs2(ind, a);
			a.x--;
			bfs(0, a, 1, 0);
			step(3, 1);
			step(4, 1);
			step(1, 1);
			step(2, 1);
			step(1, 1);
			step(4, 1);
			step(3, 1);
			step(3, 1);
			step(2, 1);
			step(1, 1);
		}
	}
	else bfs2(ind, a);
}

void Solve()
{
	for (int i = 0; i < 16; i++)
	{
		if (!equal(corFlw[cor[i]], flw[cor[i]]))
		{
			move(cor[i]);
		}
		isCor[cor[i]] = true;
	}

}

//-----------------End solve

//-----------------Game

void game()
{
	readRecord();
	iniz();
	random();
	cnt = 0;
	print();
	do {
		if (_kbhit())
		{
			int btn = _getch();
			switch (btn)
			{
				case 27:exit(0);
				case 75:step(1, 1); break;
				case 72:step(2, 1); break;
				case 77:step(3, 1); break;
				case 80:step(4, 1); break;
				case 114:game();
				case 105:Solve(); break;
			}
		}
		if (check())
		{
			printWin();
			do {
				if (_kbhit())
				{
					int btn = _getch();
					switch (btn)
					{
						case 27:exit(0);
						case 114:game();
					}
				}
			} while (1 == 1);
		}
	} while (1 == 1);
}

//-----------------End game

int main()
{
	game();
}