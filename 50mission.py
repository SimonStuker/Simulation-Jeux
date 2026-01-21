##############################################################################
############          50 missions à 2 joueurs               ##################
##############################################################################


# Librairies et variables globales
import random
import numpy as np

N_CARTES_SUR_TABLE = 4

global Pioche_cartes, Pioche_missions, joueur_actuel, etat_jeu


# Création des missions
class Mission:
    nom = ""
    contrainte = ""

    def __init__(self, nom, contrainte):
        self.contrainte = contrainte


# SIMON: Les missions comme ca sont pas evidentes a coder, elles meritent d'avoir leur propre meta fonction pour pas perdre en lisibilite
def meta_2_espacees(fx_condition, D):
    """
    Retourne une fonction qui:
    Verifie si deux cartes a distance D verifient toutes les deux une condition
    (D=1 signifie adjacentes)
    """
    def fx_contrainte(cartes):
        # SIMON: astuce ici je fais "zip" avec un decalage de D donc quand carte1 sera en N, carte 2 sera en N+D
        for carte1, carte2 in zip(cartes, cartes[D:]):
            if fx_condition(carte1) and fx_condition(carte2):
                return True
        return False

    return fx_contrainte


def meta_2_non_adjacents(fx_condition):
    """
    Retourne une fonction qui:
    Verifie si deux cartes a distance >1 verifient toutes les deux une condition
    """
    def fx_contrainte(cartes):
        # SIMON: astuce on teste meta_2_espacees pour tout D!=1. ie D=2,3,...,N_CARTES_SUR_TABLE
        for D in range(2,N_CARTES_SUR_TABLE):
            # un peu complique: meta retourne une fonction, qu'il faut ensuite appeler sur les cartes sur table
            fx_test_distance_fixe = meta_2_espacees(fx_condition, D)
            if fx_test_distance_fixe(cartes):
                return True
        return False

    return fx_contrainte

# SIMON: si j'ai bien compris, la position des cartes n'est pas importante? dans ce cas felicitations c'est correct!
# Je tente : les 4 cartes se suivent
def meta_4_valeurs_se_suivent():
    def fx_contrainte(cartes):
        L = cartes
        return (
            {1, 2, 3, 4} <= set(L)
            or {2, 3, 4, 5} <= set(L)
            or {3, 4, 5, 6} <= set(L)
            or {4, 5, 6, 7} <= set(L)
        )

    return fx_contrainte


# SIMON: vraiment pas loin ta tentative! je rends ca + generique
# SIMON: pas vraiment besoin de meta fonction ici, on pourrait directement mettre Mission(nom="truc", contrainte=fx_3_valeurs_se_suivent)
def meta_3_valeurs_se_suivent_dans_lordre():
    def fx_contrainte(cartes):
        # SIMON: pas besoin de lire l'etat global: on te donne les cartes en input deja
        # SIMON: une liste commence avec 0, pas 1 :p
        L = [x.valeurcarte for x in cartes]
        for idx in range(len(L) - 2):
            if L[idx] == L[idx+1] + 1 and L[idx] == L[idx+2] + 2:
                return True
        return False

    return fx_contrainte


# Je tente : deux couleurs ont la même somme de valeurs
def meta_somme_couleurs_egale(couleur1, couleur2):
    def fx_contrainte(cartes):
        somme_couleur1 = sum(x.valeurcarte for x in cartes if x.couleurcarte == couleur1)
        somme_couleur2 = sum(x.valeurcarte for x in cartes if x.couleurcarte == couleur2)
        return somme_couleur1 == somme_couleur2

    return fx_contrainte


# Je tente : une couleur1 a 2* la somme des valeurs d'une autre couleur2
def meta_somme_couleurs_double(couleur1, couleur2):
    def fx_contrainte(cartes):
        somme_couleur1 = sum(x.valeurcarte for x in cartes if x.couleurcarte == couleur1)
        somme_couleur2 = sum(x.valeurcarte for x in cartes if x.couleurcarte == couleur2)
        return somme_couleur1 == 2 * somme_couleur2

    return fx_contrainte

fx_valeurs_distinctes = lambda cartes: len(set(x.valeurcarte for x in cartes)) == N_CARTES_SUR_TABLE
fx_couleurs_distinctes = lambda cartes: len(set(x.couleurcarte for x in cartes)) == N_CARTES_SUR_TABLE

# Création du paquet de missions
Pioche_missions = [
    Mission(nom="Somme=10", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes) == 10),
    Mission(nom="Somme=15", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes) == 15),
    Mission(nom="Somme=18", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes) == 18),
    Mission(nom="Somme=20", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes) == 20),

    Mission(nom="R ou B", contrainte=lambda cartes: all(x.couleurcarte in ["rouge", "bleu"] for x in cartes)),
    Mission(nom="J ou B", contrainte=lambda cartes: all(x.couleurcarte in ["jaune", "bleu"] for x in cartes)),
    Mission(nom="R ou V", contrainte=lambda cartes: all(x.couleurcarte in ["rouge", "vert"] for x in cartes)),
    Mission(nom="J ou V", contrainte=lambda cartes: all(x.couleurcarte in ["jaune", "vert"] for x in cartes)),

    Mission(nom="Somme_Rouge==4", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "rouge") == 4),
    Mission(nom="Somme_Rouge==10", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "rouge") == 10),
    Mission(nom="Somme_Jaune==2", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "jaune") == 2),
    Mission(nom="Somme_Jaune==11", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "jaune") == 11),
    Mission(nom="Somme_Bleue==3", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "bleu") == 3),
    Mission(nom="Somme_Bleue==9", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "bleu") == 9),
    Mission(nom="Somme_Verte==6", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "vert") == 6),
    Mission(nom="Somme_Verte==7", contrainte=lambda cartes: sum(x.valeurcarte for x in cartes if x.couleurcarte == "vert") == 7),

    # % c'est l'operateur modulo, x.valeur = 1 [2] c'est pareil que "impair"
    Mission(nom="Tout_impair", contrainte=lambda cartes: all(x.valeur % 2 == 1 for x in cartes)),
    Mission(nom="Tout_pair", contrainte=lambda cartes: all(x.valeur % 2 == 0 for x in cartes)),
    Mission(nom="Tout>=5", contrainte=lambda cartes: all(x.valeur >= 5 for x in cartes)),
    Mission(nom="Tout<=3", contrainte=lambda cartes: all(x.valeur <= 3 for x in cartes)),

    # l'astuce pour compter le nombre de cartes qui respectent une condition, c'est de faire une sum(1 for x in cartes if condition)
    # ou bien len(list(filter(cartes, lambda x: condition)))
    Mission(nom="Trois_Vertes", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "vert") == 3),
    Mission(nom="Trois_Bleues", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "bleu") == 3),
    Mission(nom="Trois_Rouges", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "rouge") == 3),
    Mission(nom="Trois_Jaunes", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "jaune") == 3),

    # rappel: D=1 signifie adjacentes
    Mission(nom="Deux_Vertes_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleur == "vert", D=1)),
    Mission(nom="Deux_Rouges_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleur == "rouge", D=1)),
    Mission(nom="Deux_Jaunes_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleur == "jaune", D=1)),
    Mission(nom="Deux_Bleues_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleur == "bleu", D=1)),

    Mission(nom="Deux_Vertes_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleur == "vert")),
    Mission(nom="Deux_Rouges_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleur == "rouge")),
    Mission(nom="Deux_Jaunes_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleur == "jaune")),
    Mission(nom="Deux_Bleues_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleur == "bleu")),

    # rappel: D=2 signifie espacees de 1, x%2==1 signifie x impair
    Mission(nom="Deux_Vertes_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleur == "vert", D=2)),
    Mission(nom="Deux_Rouges_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleur == "rouge", D=2)),
    Mission(nom="Deux_Jaunes_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleur == "jaune", D=2)),
    Mission(nom="Deux_Bleues_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleur == "bleu", D=2)),
    Mission(nom="Deux_Impaires_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.valeurcarte % 2 == 1, D=2)),

    Mission(nom="Valeurs_Distinctes", contrainte=fx_valeurs_distinctes),
    Mission(nom="Couleurs_Distinctes", contrainte=fx_couleurs_distinctes),
    Mission(nom="Valeurs_et_Couleurs_Distinctes", contrainte=lambda cartes: fx_valeurs_distinctes(cartes) and fx_couleurs_distinctes(cartes)),

    Mission(nom="3_se_suivant_dans_lordre", contrainte=meta_3_valeurs_se_suivent_dans_lordre()),
    Mission(nom="4_se_suivent", contrainte=meta_4_valeurs_se_suivent()),

    Mission(nom="Somme_Jaune==Somme_Verte", contrainte=meta_somme_couleurs_egale("jaune", "vert")),
    Mission(nom="Somme_Jaune==Somme_Rouge", contrainte=meta_somme_couleurs_egale("jaune", "rouge")),
    Mission(nom="Somme_Bleue==Somme_Verte", contrainte=meta_somme_couleurs_egale("bleu", "vert")),
    Mission(nom="Somme_Bleue==Somme_Rouge", contrainte=meta_somme_couleurs_egale("bleu", "rouge")),

    Mission(nom="2*Somme_Verte==Somme_Jaune", contrainte=meta_somme_couleurs_double("jaune", "vert")),
    Mission(nom="2*Somme_Jaune==Somme_Rouge", contrainte=meta_somme_couleurs_double("rouge", "jaune")),
    Mission(nom="2*Somme_Bleue==Somme_Verte", contrainte=meta_somme_couleurs_double("vers", "bleu")),
    Mission(nom="2*Somme_Rouge==Somme_Bleue", contrainte=meta_somme_couleurs_double("bleu", "rouge")),
]


# Création des cartes de jeu
class Carte:
    valeurcarte = ""
    couleurcarte = ""

    def __init__(self, valeurcarte, couleurcarte):
        self.valeurcarte = valeurcarte
        self.couleurcarte = couleurcarte

    def affiche_carte(self):
        print(f"Valeur: {self.valeurcarte}, Couleur: {self.couleurcarte}")


##Affiche de la carte numéro 5
# Pioche[5].affiche_carte()

# Création du paquet de cartes
Pioche_cartes = []
for i in range(1, 8):
    for j in ("rouge", "jaune", "bleu", "vert"):
        Pioche_cartes.append(Carte(i, j))
Pioche_cartes = Pioche_cartes + Pioche_cartes
# print("DEBUG",Pioche_cartes[4].valeurcarte,Pioche_cartes[4].couleurcarte)


# Mise en place
def initialiser_jeu():
    global Pioche_cartes, Pioche_missions, etat_jeu
    # LE MELANGE NE FONCTIONNE PAS
    # Pioche_missions=random.shuffle(Pioche_missions)
    # Pioche_cartes=random.shuffle(Pioche_cartes)
    etat_jeu = {
        "Main joueur 1": Pioche_cartes[0:4],
        "Main joueur 2": Pioche_cartes[4:9],
        "Cartes sur table": Pioche_cartes[9:14],
        "Missions sur table": Pioche_missions[0:4],
        "pioche de missions": Pioche_missions[4:],
        "pioche de cartes": Pioche_cartes[14:],
        "tour": 0,
        "termine": False,
    }
    return etat_jeu


# Boucle de jeu
def jouer_partie(etat_jeu):
    while not etat_jeu["termine"]:
        jouer_un_tour(etat_jeu)


# Tour de jeu
def jouer_un_tour(etat_jeu):
    global joueur_actuel
    joueur_actuel = 1 + etat_jeu["tour"] % 2
    action = choisir_action(joueur_actuel, etat_jeu)
    appliquer_action(action, joueur_actuel, etat_jeu)
    verifier_fin_jeu(etat_jeu)
    etat_jeu["tour"] += 1


# Coups possibles du joueur
def coups_possibles(joueur, etat_jeu):
    global joueur_actuel
    CP = []
    if joueur_actuel == 1:
        for x in etat_jeu["Main joueur 1"]:
            for y in etat_jeu["Cartes sur table"]:
                if x.valeurcarte == y.valeurcarte or x.couleurcarte == y.couleurcarte:
                    CP.append([x, y])
    if joueur_actuel == 2:
        for x in etat_jeu["Main joueur 2"]:
            for y in etat_jeu("Cartes sur table"):
                if x.valeurcarte == y.couleurcarte or x.couleur == y.couleurcarte:
                    CP.append([x, y])
    return CP


# Choix d'une action
def choisir_action(joueur, etat_jeu):
    if not coups_possibles(joueur, etat_jeu) == []:
        return random.choice(coups_possibles(joueur, etat_jeu))
    ## PROBLEME SI AUCUN COUP POSSIBLE??


# Résultat de l'action
def appliquer_action(action, joueur, etat_jeu):
    action = choisir_action(
        joueur, etat_jeu
    )  # Action = couple [carte main joueur , carte sur table] jouable

    # CETTE PARTIE EST FOIREUSE, IL FAUT LA RECTIFIER

    action[0] = action[1]  # Changer la carte sur la table

    if joueur_actuel == 1:  # refaire la main du joueur
        etat_jeu["Main joueur 1"].remove(
            action[0]
        )  # Enlever la carte de la main du joueur
        etat_jeu["Main joueur 1"].append(
            etat_jeu["pioche de cartes"][0]
        )  # Ajouter la 1e carte de la pioche à la main du joueur
        etat_jeu["pioche de cartes"].remove(
            etat_jeu["pioche de cartes"][0]
        )  # Enlever la 1e carte de la pioche
    if joueur_actuel == 2:  # refaire la main du joueur
        etat_jeu["Main joueur 2"].remove(action[0])
        etat_jeu["Main joueur 2"].append(etat_jeu["pioche de cartes"][0])
        etat_jeu["pioche de cartes"].remove(etat_jeu["pioche de cartes"][0])

    for x in etat_jeu["Missions sur table"]:  # Changer les missions réussies
        if x.condition:
            etat_jeu["Missions sur table"][x] = etat_jeu["pioche de missions"][
                0
            ]  # Piocher une nouvelle mission
            etat_jeu["pioche de missions"].remove(
                etat_jeu["pioche de missions"][0]
            )  # Enlever la mission de la pioche


# Condition de victoire ou défaite et vérification de fin de jeu
def condition_victoire(etat_jeu):
    if etat_jeu["pioche de missions"] == []:
        return True
    else:
        return False


def condition_defaite(etat_jeu):
    if coups_possibles(etat_jeu) == []:
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
    initialiser_jeu()
    print("DEBUG")
    print(etat_jeu["Main joueur 1"])
    jouer_partie(etat_jeu)
    afficher_resultats(etat_jeu)


if __name__ == "__main__":
    main()
