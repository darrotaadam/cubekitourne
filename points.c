#include <stdlib.h>
#include <stdio.h>

typedef struct Point {
	long xy;
	struct Point* prev=NULLPTR;
	struct Point* nextNULLPTR;
	void (*deletePoint)(Point*);
	int (*getX)(Point*);
	int (*getY)(Point*);
} point_t;


void deletePoint(point_t* point){
	free(point);
}

point_t* creePoint(long xy){
	point_t* newPoint = (point_t*) malloc(sizeof(point_t));
	newPoint->xy = xy;
	return newPoint;
}


int getX(point_t* point){
	return (int) (point->xy & 0xffffffff00000000)>>32;
}

int getY(point_t* point){
	return (int) (point->xy & 0xffffffff);
}


///////////////////////////////////////////////


typedef struct ListePoints{
	point_t* head=NULLPTR;
	point_t* tail=NULLPTR;
	int size;
	
	// pour gerer une liste de points
	void (*ajoutePoint)(ListePoints*, long); 
	void (*debugAffiche)(ListePoints*);
	point_t* (*getByIndex)(ListePoints*, int);

	// pour gerer une liste de listes (listes imbriquées pour lignes->colonnes)
	void (*ajouteListe)(ListePoints*, ListePoints*);
	void (*debugAfficheListes)(ListePoints*);
	ListePoints* (*getListByIndex)(ListePoints*, int);

	void (*deleteList)(ListePoints*);
} pointList_t;


pointList_t creeListe(){
	pointList_t newList = (pointList_t*) malloc(sizeof(pointList_t));
	return newList;
}

void deleteList(pointList_t* list){
	enfait non // il faut delete chaque node 
	free(list);
}

void ajoutePoint(pointList_t* list, long xy){
	point_t* newPoint = creePoint(xy);
		
	// si liste vide
	if (list->size == 0){
		list->head = newPoint;
		list->tail = newPoint;
	}
	else{
		/*list->tail->next = newPoint;
		list->tail = newPoint;*/
		int i=0;
		point_t* point_iter = list->head;
		char pushed = 0;
		// les points sont mis dans l'ordre croissant selon x. On cherche le premier point plus grand que x (newPoint)
		while( point_iter != NULLPTR && point_iter.getX(point_iter) < newPoint.getX(newPoint)){
			point_iter = point_iter->next;
		}
		// si on a atteint un point avec un x plus grand, alors on cale notre nouveau point juste avant
		if(point_iter != NULLPTR){
			//si les deux xy sont identiques, on ne place pas le nouveau point
			if(newPoint->xy != point_iter->xy){
				// si list->head == list->tail : on doit séparer
				if(list->head == list->tail){
					list->head->next = newPoint;
					list->tail = newPoint;
					newPoint->prev = list->head;
				}
				// s'il y a 2 ou plus éléments, et qu'on est au ième
				else{ 
					point_iter->prev->next = newPoint;
					newPoint->prev = point_iter->prev;
					newPoint->next = point_iter;
					point_iter->prev = newPoint;
				}
			}		
		}
		// si on a parcouru tous les points sans trouver de plus grand : alors le point actuel vaut NULLPTR
		// on ajoute donc notre point a la fin
		else{
			newPoint->prev = list->tail;
			list->tail->next = newPoint;
			list->tail = newPoint;
		}

	}

	list->size++;
}

point_t* getByIndex(pointList_t* list, int i){
	if( i+1 >= list->size){
		returnValue = NULLPTR;
	}
	else{
		returnValue = list->head;
		for(int j=0; j<i; j++){
			returnValue = returnValue->next;
		}
	}
	return returnValue;
}


void debugAffiche(pointList_t* list){
	point_p* point;
	for(int i=0; i<list->size; i++){
		point = list.getByIndex(&list, i);
		printf(" [%l|%p] ->", point->xy, point);
	}
	
}


void ajouteListe(pointList_t* list, pointList_t* newList){
	// si liste vide
	if (list->size == 0){
		list->head = newList;
		list->tail = newList;
	}
	else{
		list->tail->next = newList;
		list->tail = newList;
	}

	list->size++;	
}

void debugAfficheListes(pointList_t* list){
	ListPoints* list_iter;
	for(int i=0; i<list->size; i++){
		list_iter = list.getListByIndex(&list, i);
		printf("\n[[%p]] : \n\t",list_iter);
		list_iter.debugAffiche(&list_iter);
	}
	printf("\n");
}

pointList_t* getListByIndex(pointList_t* list, int i){
	if( i+1 >= list->size){
		returnValue = NULLPTR;
	}
	else{
		returnValue = list->head;
		for(int j=0; j<i; j++){
			returnValue = returnValue->next;
		}
	}
	return returnValue;
}


