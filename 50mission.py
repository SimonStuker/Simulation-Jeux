    cartes_sur_table = etat_jeu.cartes_sur_table
      
    etat_jeu.defausse.append(cartes_sur_table[idx_table]) #Placer la carte dans la défausse
    carte_jouee = main_joueur.pop(idx_main) # Enlever la carte de la main du joueur
    if len(pioche) > 0:
        nouvelle_carte = pioche.pop() # Enlever la carte de la pioche
        main_joueur.append(nouvelle_carte)  # Ajouter la carte de piochee a la main du joueur

    cartes_sur_table[idx_table] = carte_jouee # Remplacer la carte sur table
