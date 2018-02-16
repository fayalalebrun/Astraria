package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/7/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Group;
import com.badlogic.gdx.scenes.scene2d.ui.Window;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.*;
import com.mygdx.game.playback.PlayBackBody;
import com.mygdx.game.playback.PlayBackScreen;

public class MenuWidget extends MenuBar {

    private Menu optionsMenu;
    private Menu aboutMenu;
    private MenuItem colorToolMenuItem;
    private MenuItem creditsMenuItem;
    private MenuItem bodyScaleMenuItem;
    private BodyScale bodyScale;
    private MenuItem camSpeedItem;
    private CamSpeed camSpeed;

    public MenuWidget(Group uiGroup, PlayBackScreen playBackScreen){
        super();

        final ColorTools colorTools = new ColorTools(playBackScreen);
        uiGroup.addActor(colorTools);
        colorTools.setVisible(false);

        bodyScaleMenuItem = new MenuItem("Object scale");
        bodyScale = new BodyScale(playBackScreen);
        uiGroup.addActor(bodyScale);
        bodyScale.setVisible(false);

        camSpeedItem = new MenuItem("Camera speed");
        camSpeed = new CamSpeed(playBackScreen);
        uiGroup.addActor(camSpeed);
        camSpeed.setVisible(false);





        optionsMenu = new Menu("Options");
        aboutMenu = new Menu("About");
        colorToolMenuItem = new MenuItem("Color tools", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (colorTools.isVisible()){
                    colorTools.setVisible(false);
                }else {
                    colorTools.setVisible(true);
                }
            }
        });

        bodyScaleMenuItem.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (bodyScale.isVisible()){
                    bodyScale.setVisible(false);
                }else {
                    bodyScale.setVisible(true);
                }
            }
        });

        camSpeedItem.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {

                    if (camSpeed.isVisible()){
                        camSpeed.setVisible(false);
                    }else {
                        camSpeed.setVisible(true);
                    }
            }
        });

        creditsMenuItem = new MenuItem("Credits");

        PopupMenu popupMenu = new PopupMenu();
        popupMenu.add(new CreditsWindow());


        optionsMenu.addItem(colorToolMenuItem);
        optionsMenu.addItem(bodyScaleMenuItem);
        aboutMenu.addItem(creditsMenuItem);
        optionsMenu.addItem(camSpeedItem);
        optionsMenu.pack();
        aboutMenu.pack();

        creditsMenuItem.setSubMenu(popupMenu);

        addMenu(optionsMenu);
        addMenu(aboutMenu);
    }



}
