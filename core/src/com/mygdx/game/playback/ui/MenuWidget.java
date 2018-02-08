package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/7/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.widget.*;

public class MenuWidget extends MenuBar {

    private Menu optionsMenu;
    private Menu aboutMenu;
    private MenuItem colorToolMenuItem;
    private MenuItem creditsMenuItem;

    public MenuWidget(){
        super();

        optionsMenu = new Menu("Options");
        aboutMenu = new Menu("About");
        colorToolMenuItem = new MenuItem("Color tools", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {

            }
        });

        creditsMenuItem = new MenuItem("Credits", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {

            }
        });

        PopupMenu popupMenu = new PopupMenu();
        popupMenu.add(new VisWindow("Credits"));

        optionsMenu.addItem(colorToolMenuItem);
        aboutMenu.addItem(creditsMenuItem);

        creditsMenuItem.setSubMenu(popupMenu);

        addMenu(optionsMenu);
        addMenu(aboutMenu);
    }



}
