############################################################################## 
############          50 missions à 2 joueurs               ##################
##############################################################################


# Librairies et variables globales
import random
import numpy as np
global Pioche_cartes, Pioche_missions, joueur_actuel

# Création des missions
class Mission:
    contrainte=""
    def __init__(self, contrainte):
        self.contrainte=  contrainte
        
# QUINTEN-STYLE :" c'est une meta-fonction, c'est une fonction qui donne une fonction N^(N^N) en maths a peu pres"
def meta_couleurs(couleurs_autorisees):
    def fx_contrainte(cartes):
        return all(cartes.couleurcarte in couleurs_autorisees)
    return fx_contrainte

#Je tente : somme d'une couleur donnée
def meta_somme_couleur(couleur,R):
    def fx_contrainte(cartes):
        return sum([x.valeurcarte for x in cartes])==R
    return fx_contrainte

#Je tente : tous les chiffres dans un ensemble
def meta_valeurs(valeurs_autorisees):
    def fx_contrainte(cartes):
        return all(cartes.valeurcarte in valeurs_autorisees)
    return fx_contrainte

#Je tente : nombre de cartes d'une couleur
def meta_nombre_couleur(couleur,nombre):
    def fx_contrainte(cartes):
        return cartes.couleurcartecarte(couleur)==nombre   ##PAS CERTAIN DU TOUT
    return fx_contrainte
        

# Création du paquet de missions
Pioche_missions=np.zeros(50)
# QUINTEN : "voila comment tu ferais "a l'arrache""
Pioche_missions[0]=Mission(nom="Somme=10", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes)==10)
Pioche_missions[1]=Mission(nom="Somme=15", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes)==15)
Pioche_missions[2]=Mission(nom="Somme=18", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes)==18)
Pioche_missions[3]=Mission(nom="Somme=20", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes)==20)

# QUINTEN : "voila comment tu ferais avec des "meta fonctions", plus propre"
Pioche_missions[4]=Mission(nom="R ou B", contrainte=meta_couleurs(['rouge','bleu']))
Pioche_missions[5]=Mission(nom="J ou B", contrainte=meta_couleurs(['jaune','bleu']))
Pioche_missions[6]=Mission(nom="R ou V", contrainte=meta_couleurs(['rouge','vert']))
Pioche_missions[7]=Mission(nom="J ou V", contrainte=meta_couleurs(['jaune','vert']))

Pioche_missions[8]=Mission(nom="Somme_Rouge==4",contrainte=meta_somme_couleur('rouge',4))
Pioche_missions[9]=Mission(nom="Somme_Rouge==10",contrainte=meta_somme_couleur('rouge',10))
Pioche_missions[10]=Mission(nom="Somme_Jaune==2",contrainte=meta_somme_couleur('jaune',2))
Pioche_missions[11]=Mission(nom="Somme_Jaune==11",contrainte=meta_somme_couleur('jaune',11))
Pioche_missions[12]=Mission(nom="Somme_Bleue==3",contrainte=meta_somme_couleur('bleu',3))
Pioche_missions[13]=Mission(nom="Somme_Bleue==9",contrainte=meta_somme_couleur('bleu',9))
Pioche_missions[14]=Mission(nom="Somme_Verte==6",contrainte=meta_somme_couleur('vert',6))
Pioche_missions[15]=Mission(nom="Somme_Verte==7",contrainte=meta_somme_couleur('vert',7))

Pioche_missions[16]=Mission(nom="Tout_impair", contrainte=meta_couleurs([1,3,5,7]))
Pioche_missions[17]=Mission(nom="Tout_pair", contrainte=meta_couleurs([2,4,6]))
Pioche_missions[18]=Mission(nom="Tout>=5", contrainte=meta_couleurs([5,6,7]))
Pioche_missions[19]=Mission(nom="Tout<=3", contrainte=meta_couleurs([1,2,3]))

Pioche_missions[20]=Mission(nom="Trois_Vertes", contrainte= meta_nombre_couleur('vert',3))
Pioche_missions[21]=Mission(nom="Trois_Bleues", contrainte= meta_nombre_couleur('bleu',3))
Pioche_missions[22]=Mission(nom="Trois_Rouges", contrainte= meta_nombre_couleur('rouge',3))
Pioche_missions[23]=Mission(nom="Trois_Jaunes", contrainte= meta_nombre_couleur('jaune',3))

Pioche_missions[24]=Mission("Deux_Vertes_adjacentes")
Pioche_missions[25]=Mission("Deux_Rouges_adjacentes")
Pioche_missions[26]=Mission("Deux_Jaunes_adjacentes")
Pioche_missions[27]=Mission("Deux_Bleues_adjacentes")

Pioche_missions[28]=Mission("Deux_Vertes_espacees")
Pioche_missions[29]=Mission("Deux_Rouges_espacees")
Pioche_missions[30]=Mission("Deux_Jaunes_espacees")
Pioche_missions[31]=Mission("Deux_Bleues_espacees")

Pioche_missions[32]=Mission("Deux_Vertes_espacees_de_1")
Pioche_missions[33]=Mission("Deux_Rouges_espacees_de_1")
Pioche_missions[34]=Mission("Deux_Jaunes_espacees_de_1")
Pioche_missions[35]=Mission("Deux_Bleues_espacees_de_1")
Pioche_missions[36]=Mission("Deux_Impaires_espacees_de_1")

Pioche_missions[37]=Mission("Valeurs_Distinctes")
Pioche_missions[38]=Mission("Couleurs_Distinctes")
Pioche_missions[39]=Mission("Valeurs_et_Couleurs_Distinctes")

Pioche_missions[40]=Mission("3_se_suivant_dans_lordre")
Pioche_missions[41]=Mission("4_se_suivent")

Pioche_missions[42]=Mission("Somme_Jaune==Somme_Verte")
Pioche_missions[43]=Mission("Somme_Jaune==Somme_Rouge")
Pioche_missions[44]=Mission("Somme_Bleue==Somme_Verte")
Pioche_missions[45]=Mission("Somme_Bleue==Somme_Rouge")

Pioche_missions[46]=Mission("Somme_Jaune==2*Somme_Verte")
Pioche_missions[47]=Mission("2*Somme_Jaune==Somme_Rouge")
Pioche_missions[48]=Mission("2*Somme_Bleue==Somme_Verte")
Pioche_missions[49]=Mission("Somme_Bleue==2*Somme_Rouge")
        
# Création des cartes de jeu
class Carte:
    valeurcarte=""
    couleurcarte=""
    def __init__(self, valeurcarte,couleurcarte):
        self.valeurcarte=  valeurcarte
        self.couleurcarte=  couleurcarte
    def affiche_carte(self):
        print(f"Valeur: {self.valeurcarte}, Couleur: {self.couleurcarte}")
##Affiche de la carte numéro 5
#Pioche[5].affiche_carte()

# Création du paquet de cartes
Pioche_cartes=[]
for i in range(1,8):
    for j in ('rouge','jaune','bleu','vert'):
        Pioche_cartes.append(Carte(i,j))
Pioche_cartes=Pioche_cartes+Pioche_cartes

# Mise en place
def initialiser_jeu():
    Pioche_missions=random.shuffle(Pioche_missions)
    Pioche_cartes=random.shuffle(Pioche_cartes)
    etat_jeu = {
        "Main joueur 1" : Pioche_cartes[0,1,2,3],
        "Main joueur 2" : Pioche_cartes[4,5,6,7],
        "Cartes sur table" : Pioche_cartes[7,8,9,10],
        "Missions sur table" :  Pioche_missions[0,1,2,3],
        "pioche de missions": Pioche_missions[4:],
        "pioche de cartes": Pioche_cartes[11:],
        "tour": 0,
        "termine": False
    }
    return etat_jeu

# Boucle de jeu
def jouer_partie(etat_jeu):
    while not etat_jeu["termine"]:
        jouer_un_tour(etat_jeu)
        
# Tour de jeu
def jouer_un_tour(etat_jeu):
    joueur_actuel = 1+etat_jeu["tour"]%2
    action = choisir_action(joueur_actuel, etat_jeu)
    appliquer_action(action, joueur_actuel, etat_jeu)
    verifier_fin_jeu(etat_jeu)
    etat_jeu["tour"] += 1
    
# Coups possibles du joueur
def coups_possibles(joueur,etat_jeu):
    CP=[]
    for x in etat_jeu[joueur_actuel]:
        for y in etat_jeu("Missions sur table"):
            if x.valeurcarte==y.valeurcarte or x.couleur==y.couleur:
                CP.append([x,y])
    return CP
    
# Choix d'une action
def choisir_action(joueur, etat_jeu):
    if not coups_possibles(joueur,etat_jeu)==[]:
        return random. choice(coups_possibles(joueur,etat_jeu))
    ## PROBLEME SI AUCUN COUP POSSIBLE??

# Résultat de l'action
def appliquer_action(action, joueur, etat_jeu):
    action =  choisir_action(joueur, etat_jeu)
    action[1]=action[2]                        #Changer la carte sur la table
    if joueur_actuel == 1:  #refaire la main du joueur
        etat_jeu["Main joueur 1"].remove(action[1]) #Enlever la carte de la main du joueur
        etat_jeu["Main joueur 1"].append(etat_jeu["pioche de cartes"][0]) #Ajouter la 1e carte de la pioche à la main du joueur
        etat_jeu["pioche de cartes"].remove(etat_jeu["pioche de cartes"][0]) #Enlever la 1e carte de la pioche
    if joueur_actuel == 2:  #refaire la main du joueur
        etat_jeu["Main joueur 2"].remove(action[1])
        etat_jeu["Main joueur 2"].append(etat_jeu["pioche de cartes"][0])
        etat_jeu["pioche de cartes"].remove(etat_jeu["pioche de cartes"][0])
        
    for x in  etat_jeu[ "Missions sur table" ]:                            #Changer les missions réussies
        if x.condition:
            etat_jeu[ "Missions sur table" ][x]=etat_jeu[ "pioche de missions" ][0] #Piocher une nouvelle mission
            etat_jeu[ "pioche de missions" ].remove(etat_jeu[ "pioche de missions" ][0]) #Enlever la mission de la pioche
    
# Condition de victoire ou défaite et vérification de fin de jeu
def condition_victoire(etat_jeu):
    if etat_jeu["pioche de missions"]==[]:
        return True
    else:
        return False

def condition_defaite(etat_jeu):
    if coups_possibles(etat_jeu)==[]:
        return True
    else:
        return False
    
def verifier_fin_jeu(etat_jeu):
    if condition_victoire(etat_jeu):
        etat_jeu["termine"] = True
    if condition_defaite(etat_jeu):
        etat_jeu["termine"] = True
        
# Affichage du résultat
def afficher_resultats(etat_jeu):
    print("Partie terminée !")

# Boucle principale 

def main():
    etat_jeu = initialiser_jeu()
    jouer_partie(etat_jeu)
    afficher_resultats(etat_jeu)

if __name__ == "__main__":
    main()
    
    




