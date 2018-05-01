package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/7/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Group;
import com.badlogic.gdx.scenes.scene2d.ui.Window;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.Array;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.*;
import com.kotcrab.vis.ui.widget.file.FileChooser;
import com.kotcrab.vis.ui.widget.file.FileChooserAdapter;
import com.kotcrab.vis.ui.widget.file.FileTypeFilter;
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
    private Menu fileMenu;

    public MenuWidget(final Group uiGroup, final PlayBackScreen playBackScreen){
        super();

        final ColorTools colorTools = new ColorTools(playBackScreen);
        uiGroup.addActor(colorTools);
        colorTools.setVisible(false);

        final CustomFileChooser fileChooser = new CustomFileChooser(FileChooser.Mode.OPEN);
        fileChooser.setSelectionMode(FileChooser.SelectionMode.FILES);
        fileChooser.fadeOut(0);

        final FileTypeFilter fileTypeFilter = new FileTypeFilter(true);
        fileTypeFilter.addRule("NBD files (*.txt)","nbd");

        fileChooser.setFileTypeFilter(fileTypeFilter);
        fileChooser.setListener(new FileChooserAdapter() {

            @Override
            public void selected (Array<FileHandle> file) {
                if (file.get(0).file().exists()){
                    try {
                        //change simulation file
                    }catch (Exception e){
                        e.printStackTrace();
                    }
                }
            }

            @Override
            public void canceled() {
                super.canceled();
            }
        });
        uiGroup.addActor(fileChooser);

        bodyScaleMenuItem = new MenuItem("Object scale");
        bodyScale = new BodyScale(playBackScreen);
        uiGroup.addActor(bodyScale);
        bodyScale.setVisible(false);

        camSpeedItem = new MenuItem("Camera speed");
        camSpeed = new CamSpeed(playBackScreen);
        uiGroup.addActor(camSpeed);
        camSpeed.setVisible(false);

        MenuItem fullScreenToggle = new MenuItem("Toggle fullscreen");


        /*

        multiplexer.addProcessor(new InputAdapter(){
            @Override
            public boolean keyDown(int keycode) {
                if (keycode==Input.Keys.F){
                    if(!Gdx.graphics.isFullscreen()){
                        Gdx.graphics.setFullscreenMode(Gdx.graphics.getDisplayMode());
                        System.out.println("S");
                    } else {
                        Gdx.graphics.setWindowedMode(800,600);
                    }
                    return true;
                }
                System.out.println("k");
                return false;
            }
        });
         */




        fileMenu = new Menu("Astraria");
        optionsMenu = new Menu("Options");
        aboutMenu = new Menu("About");

        MenuItem fileSelect = new MenuItem("Open Simulation", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                uiGroup.addActor(fileChooser.fadeIn());            }
        });

        MenuItem gotoLaucnher = new MenuItem("Change module", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                //do something
            }
        });

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

        fullScreenToggle.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
              playBackScreen.toggleFullscreen();
            }
        });

        creditsMenuItem = new MenuItem("Credits");

        PopupMenu popupMenu = new PopupMenu();
        popupMenu.add(new CreditsWindow());


        fileMenu.addItem(fileSelect);
        fileMenu.addItem(gotoLaucnher);
        optionsMenu.addItem(colorToolMenuItem);
        optionsMenu.addItem(bodyScaleMenuItem);
        aboutMenu.addItem(creditsMenuItem);
        optionsMenu.addItem(camSpeedItem);
        optionsMenu.addItem(fullScreenToggle);
        optionsMenu.pack();
        aboutMenu.pack();

        creditsMenuItem.setSubMenu(popupMenu);

        addMenu(fileMenu);
        addMenu(optionsMenu);
        addMenu(aboutMenu);
    }



}
