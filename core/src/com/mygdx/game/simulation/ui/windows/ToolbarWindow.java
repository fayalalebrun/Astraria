package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Group;
import com.badlogic.gdx.scenes.scene2d.ui.Button;
import com.badlogic.gdx.scenes.scene2d.ui.Image;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.Drawable;
import com.badlogic.gdx.scenes.scene2d.utils.TextureRegionDrawable;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.ListView;
import com.kotcrab.vis.ui.widget.VisImage;
import com.kotcrab.vis.ui.widget.VisImageButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.Boot;
import net.dermetfan.utils.Pair;

import java.util.ArrayList;

/**
 * Created by Fran on 4/20/2018.
 */
public class ToolbarWindow extends VisWindow{
    private Group listGroup;
    private Group infoGroup;


    public ToolbarWindow(Group listGroup, Group infoGroup) {
        super("Tools");
        this.listGroup = listGroup;
        this.infoGroup = infoGroup;
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        VisImageButton listButton = new VisImageButton(convertToDrawable(Boot.manager.get("icons/list.png", Texture.class)),"Object List");
        setButtonAction(listButton, listGroup);

        add(listButton);
    }

    private void setButtonAction(VisImageButton button, final Group windowGroup){
        button.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                windowGroup.setVisible(!windowGroup.isVisible());
            }
        });
    }

    private TextureRegionDrawable convertToDrawable(Texture texture){
        return new TextureRegionDrawable(new TextureRegion(texture));
    }
}
