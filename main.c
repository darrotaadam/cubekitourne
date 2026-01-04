#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include "point.h"
#include<unistd.h>


// en caractères/lignes
static int WIDTH = 500;
static int HEIGHT = 500;

static int status=1;


static int[][3] scene = {
	{0.5, 0.5, 1},
	{0.5, -0.5, 1},
	{-0.5, 0.5, 1},
	{-0.5, -0.5, 1}
};


// on prend en compte une liste de vertex (positions x, y, z) a représenter en 2d
// pour chaque vertex : on calcule le point x_,y_ sur l'écran avec xy(x, y , z)
// on va stocker la liste des points dans une liste chainée 
// * on pourrait trier la liste chainée pendant sa construction, à chaque ajout d'un point, on le place après tous les points 
// * dont x est inférieur. On pourra pas trier par rapport à y, mais on aura une semi optimisation
// chaque point doit être placé dans le terminal, un espace " " si rien
// sur chaque ligne, il faut mettre (écrire) les points dans l'ordre (bah oui gros malin)
// on peut donc faire deux boucles imbriquées : lignes => caractères et vérifier si un point a cette coordonnée


void exit_(){
	exit(0);
}






long xy(int x, int y, int z){
	long x_ = x/z ;
	int y_ = y/z;
	return x_ << 32 & y_;	// standard pour passer les coordonnées x,y sur une seule valeur 64 bits
}


 



int main(int argc, char** argv){

	signal(SIGINT, exit_);
	
	int i;
	long xy_coords;
	pointList_t* list = creeListe();
	while (status == 1){
		// cree et stocke les points de la scène
		for(i=0; i< sizeof(scene)/sizeof(scene[0]); i++ ){
			xy_coords = xy(scene[i][0], scene[i][1], scene[i][2]);
			list.ajoutePoint(&list, xy_coords);
		}

		sleep(1);
	}
	



	return 0;
}
