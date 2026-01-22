##############################################################################
############          50 missions à 2 joueurs               ##################
##############################################################################


# Librairies et variables globales
import random
import numpy as np

N_CARTES_EN_MAIN = 4
N_MISSIONS_SUR_TABLE = 4
N_CARTES_SUR_TABLE = 4
DEBUG = True

global Pioche_cartes, Pioche_missions, joueur_actuel, etat_jeu


# Création des missions
class Mission:
    nom = ""
    contrainte = lambda: print("Cette mission n'a pas de contrainte (contrainte par defaut)")

    def __init__(self, nom, contrainte):
        self.nom = nom
        self.contrainte = contrainte

    def check_fini(self, cartes_sur_table):
        return self.contrainte(cartes_sur_table)


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
            if (L[idx] == L[idx+1] + 1 and L[idx] == L[idx+2] + 2) or (L[idx] == L[idx+1] - 1 and L[idx] == L[idx+2] - 2):
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

    # % c'est l'operateur modulo, x.valeurcarte = 1 [2] c'est pareil que "impair"
    Mission(nom="Tout_impair", contrainte=lambda cartes: all(x.valeurcarte % 2 == 1 for x in cartes)),
    Mission(nom="Tout_pair", contrainte=lambda cartes: all(x.valeurcarte % 2 == 0 for x in cartes)),
    Mission(nom="Tout>=5", contrainte=lambda cartes: all(x.valeurcarte >= 5 for x in cartes)),
    Mission(nom="Tout<=3", contrainte=lambda cartes: all(x.valeurcarte <= 3 for x in cartes)),

    # l'astuce pour compter le nombre de cartes qui respectent une condition, c'est de faire une sum(1 for x in cartes if condition)
    # ou bien len(list(filter(cartes, lambda x: condition)))
    Mission(nom="Trois_Vertes", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "vert") == 3),
    Mission(nom="Trois_Bleues", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "bleu") == 3),
    Mission(nom="Trois_Rouges", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "rouge") == 3),
    Mission(nom="Trois_Jaunes", contrainte=lambda cartes: sum(1 for x in cartes if x.couleurcarte == "jaune") == 3),

    # rappel: D=1 signifie adjacentes
    Mission(nom="Deux_Vertes_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "vert", D=1)),
    Mission(nom="Deux_Rouges_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "rouge", D=1)),
    Mission(nom="Deux_Jaunes_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "jaune", D=1)),
    Mission(nom="Deux_Bleues_adjacentes", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "bleu", D=1)),

    Mission(nom="Deux_Vertes_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleurcarte == "vert")),
    Mission(nom="Deux_Rouges_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleurcarte == "rouge")),
    Mission(nom="Deux_Jaunes_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleurcarte == "jaune")),
    Mission(nom="Deux_Bleues_espacees", contrainte=meta_2_non_adjacents(lambda x: x.couleurcarte == "bleu")),

    # rappel: D=2 signifie espacees de 1, x%2==1 signifie x impair
    Mission(nom="Deux_Vertes_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "vert", D=2)),
    Mission(nom="Deux_Rouges_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "rouge", D=2)),
    Mission(nom="Deux_Jaunes_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "jaune", D=2)),
    Mission(nom="Deux_Bleues_espacees_de_1", contrainte=meta_2_espacees(lambda x: x.couleurcarte == "bleu", D=2)),
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
    global Pioche_cartes, Pioche_missions, etat_jeu, joueur_actuel
    random.shuffle(Pioche_missions)
    random.shuffle(Pioche_cartes)
    # SIMON: je laisse un commentaire sur github pour expliquer le souci subtil
    joueur_actuel = 0
    etat_jeu = {
        "Main joueur 0": [Pioche_cartes.pop() for _ in range(N_CARTES_EN_MAIN)],
        "Main joueur 1": [Pioche_cartes.pop() for _ in range(N_CARTES_EN_MAIN)],
        "Cartes sur table": [Pioche_cartes.pop() for _ in range(N_CARTES_SUR_TABLE)],
        "Missions sur table": [Pioche_missions.pop() for _ in range(N_MISSIONS_SUR_TABLE)],
        "pioche de cartes": Pioche_cartes,
        "pioche de missions": Pioche_missions,
        "tour": 0,
        "termine": False,
        "defausse" : [],
        "sprint final": False,
    }
    return etat_jeu


# Boucle de jeu
def jouer_partie(etat_jeu):
    while not etat_jeu["termine"]:
        jouer_un_tour(etat_jeu)

def debug_tour(etat_jeu, action, joueur):
    def _fmt_carte(c):
        return f"{c.valeurcarte}-{c.couleurcarte:<5}"

    def _fmt_list_cartes(lst):
        return " | ".join([_fmt_carte(c) for c in lst])

    n_tour = etat_jeu["tour"]
    cartes = etat_jeu["Cartes sur table"]
    missions = etat_jeu["Missions sur table"]
    pioche_cartes = etat_jeu["pioche de cartes"]
    pioche_missions = etat_jeu["pioche de missions"]
    mains = [
        etat_jeu["Main joueur 0"],
        etat_jeu["Main joueur 1"],
    ]

    title = f" TOUR {n_tour} "
    bar = "═" * max(40, len(title) + 10)
    print(f"╔{bar}╗")
    print(f"║{title:^{len(bar)}}║")
    print(f"╚{bar}╝")

    print("┌─ Action actuelle")
    print(f"│ Joueur: {joueur}")
    print(f"│ Action: {_fmt_carte(mains[joueur][action[0]])} -> {_fmt_carte(cartes[action[1]])}")
    print("└────────────────────────")

    print("┌─ Mains des joueurs")
    print(f"│   Joueur 0 ( {len(mains[0])} cartes )")
    print(f"│     {_fmt_list_cartes(mains[0])}")
    print(f"│   Joueur 1 ( {len(mains[1])} cartes )")
    print(f"│     {_fmt_list_cartes(mains[1])}")
    print("└────────────────────────")

    print(f"┌─ Cartes sur table ( {len(cartes)} cartes )")
    print(f"│     {_fmt_list_cartes(cartes)}")
    print("└────────────────────────")

    print("┌─ Missions sur table")
    for m in missions:
        print(f"│    - {m.nom}")
    print("└────────────────────────")

    print("┌─ Pioches")
    print(f"│   Cartes   : {len(pioche_cartes):3} restantes")
    print(f"│   Missions : {len(pioche_missions):3} restantes")
    print("└────────────────────────")


# Tour de jeu
def jouer_un_tour(etat_jeu):
    global joueur_actuel
    action = choisir_action(joueur_actuel, etat_jeu)
    if DEBUG:
        debug_tour(etat_jeu, action, joueur_actuel)
    appliquer_action(action, joueur_actuel, etat_jeu)
    verifier_fin_jeu(etat_jeu)
    joueur_actuel = (1 + etat_jeu["tour"]) % 2
    etat_jeu["tour"] += 1
    
    # Si 25 missions ont été réalisées pour la premiere fois, mélange la défausse à la pioche et reprendre.
    if len(Pioche_missions)<25 and not etat_jeu["sprint_final"]:
        etat_jeu["sprint_final"]=True
        random.shuffle(etat_jeu("defausse"))
        etat_jeu["pioche de cartes"]=etat_jeu["pioche de cartes"]+etat_jeu["defausse"]
        

# Coups possibles du joueur
def coups_possibles(joueur, etat_jeu):
    """
    Retourne une liste de tuples (idx_1, idx_2)
    idx_1: indice de la carte jouable dans la main du joueur
    idx_2: indice de la carte sur la table remplacable
    """
    coups = []
    if joueur == 0:
        for idx_x, x in enumerate(etat_jeu["Main joueur 0"]):
            for idx_y, y in enumerate(etat_jeu["Cartes sur table"]):
                if x.valeurcarte == y.valeurcarte or x.couleurcarte == y.couleurcarte:
                    coups.append((idx_x, idx_y))

    else:
        assert joueur == 1, f"ERREUR: Bizarre, le joueur ne vaut ni 0 ni 1: {joueur}"
        for idx_x, x in enumerate(etat_jeu["Main joueur 1"]):
            for idx_y, y in enumerate(etat_jeu["Cartes sur table"]):
                if x.valeurcarte == y.valeurcarte or x.couleurcarte == y.couleurcarte:
                    coups.append((idx_x, idx_y))

    return coups


# Choix d'une action
def choisir_action(joueur, etat_jeu):
    coups = coups_possibles(joueur, etat_jeu)
    # Facon propre de s'assurer (assert) qu'on a au moins un coup
    assert len(coups) > 0, "ERREUR: Aucun coup possible: le jeu aurait du s'arreter"
    return random.choice(coups)


# Résultat de l'action
def appliquer_action(action, joueur, etat_jeu):
    # SIMON: je laisse un commentaire sur github pour expliquer le souci subtil
    idx_main, idx_table = action

    pioche = etat_jeu["pioche de cartes"]
    main_joueur = etat_jeu[f"Main joueur {joueur}"]
    etat_jeu["defausse"].append(idx_main) #Placer la carte dans la défausse
    carte_jouee = main_joueur.pop(idx_main) # Enlever la carte de la main du joueur
    if len(pioche) > 0:
        nouvelle_carte = pioche.pop() # Enlever la carte de la pioche
        main_joueur.append(nouvelle_carte)  # Ajouter la carte de piochee a la main du joueur

    cartes_sur_table = etat_jeu["Cartes sur table"]
    cartes_sur_table[idx_table] = carte_jouee # Remplacer la carte sur table

    missions = etat_jeu["Missions sur table"]
    liste_fini = [m.check_fini(cartes_sur_table) for m in missions] # il ne faut appeler check_fini que une fois
    missions_finies = [m for (m, fini) in zip(missions, liste_fini) if fini]
    missions_restantes = [m for (m, fini) in zip(missions, liste_fini) if not fini]
    assert len(missions_finies) + len(missions_restantes) == len(missions), "ERREUR: Bizarre le filtre des missions n'a pas bien fonctionne!"

    if DEBUG:
        if len(missions_finies) > 0:
            print(f"┌─ Missions tout juste finies ({len(missions_finies)} missions)")
            for m in missions_finies:
                print(f"│    - {m.nom}")
            print("└────────────────────────")
        else:
            print("Aucune mission finie ce tour ci.")

    etat_jeu["Missions sur table"] = missions_restantes.copy()

    # Completer les missions sur table
    missions = etat_jeu["Missions sur table"]
    pioche_missions = etat_jeu["pioche de missions"]
    while len(missions) < N_MISSIONS_SUR_TABLE and len(pioche_missions) > 0:
        nouvelle_mission = pioche_missions.pop()
        missions.append(nouvelle_mission)


# Condition de victoire ou défaite et vérification de fin de jeu
def condition_victoire(etat_jeu):
    # SIMON: petit commentaire sur github
    return len(etat_jeu["pioche de missions"]) == 0


def condition_defaite(etat_jeu):
    # SIMON: petit commentaire sur github
    coups_j1 = coups_possibles(0, etat_jeu)
    coups_j2 = coups_possibles(1, etat_jeu)
    #Vérifier si le joueur suivant ne peut pas jouer. Dans ce cas c'est la défaite.
    return (joueur_actuel==1 and len(coups_j1) == 0) or (joueur_actuel==0 and len(coups_j2) == 0) 


def verifier_fin_jeu(etat_jeu):
    etat_jeu["termine"] = condition_victoire(etat_jeu) or condition_defaite(etat_jeu)


# Affichage du résultat
def afficher_resultats(etat_jeu):
    print("Partie terminée !")


# Boucle principale
def main():
    if DEBUG:
        print("=================================")
        print("= Mode DEBUG actif (DEBUG=True) =")
        print("=================================")

    initialiser_jeu()
    jouer_partie(etat_jeu)
    afficher_resultats(etat_jeu)


if __name__ == "__main__":
    main()
