package com.mygdx.game.playback;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.PerspectiveCamera;
import com.badlogic.gdx.graphics.g3d.ModelBatch;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.decals.CameraGroupStrategy;
import com.badlogic.gdx.graphics.g3d.decals.Decal;
import com.badlogic.gdx.graphics.g3d.decals.DecalBatch;
import com.badlogic.gdx.graphics.g3d.decals.DecalMaterial;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;

import java.lang.reflect.Array;
import java.util.ArrayList;

/**
 * Created by fraayala19 on 1/11/18.
 */
public class PlayBackScreen extends BaseScreen{

    private PerspectiveCamera cam;

    private DecalBatch decalBatch;

    private ArrayList<Decal> toRender;


    public PlayBackScreen(Boot boot, String[] arg) {
        super(boot);

        cam = new PerspectiveCamera(67, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
        decalBatch = new DecalBatch(new CameraGroupStrategy(cam));
        toRender = new ArrayList<Decal>();
        
    }

    @Override
    public void show() {

    }

    @Override
    public void render(float delta) {

    }

    @Override
    public void resize(int width, int height) {

    }

    @Override
    public void pause() {

    }

    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void dispose() {
        decalBatch.dispose();
    }
}
