package com.mygdx.game.playback;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.PerspectiveCamera;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.graphics.g3d.ModelBatch;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.decals.CameraGroupStrategy;
import com.badlogic.gdx.graphics.g3d.decals.Decal;
import com.badlogic.gdx.graphics.g3d.decals.DecalBatch;
import com.badlogic.gdx.graphics.g3d.decals.DecalMaterial;
import com.badlogic.gdx.graphics.g3d.utils.FirstPersonCameraController;
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

    private ArrayList<ModelInstance> toRender;

    private Decal d;

    FirstPersonCameraController camControl;


    public PlayBackScreen(Boot boot, String[] arg) {
        super(boot);



    }

    @Override
    public void show() {
        cam = new PerspectiveCamera(67, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

        cam.near = 0.1f;
        cam.far = 100000f;
        cam.position.set(10000,10,10);
        cam.lookAt(0,0,0);
        cam.update();



        camControl = new FirstPersonCameraController(cam);
        Gdx.input.setInputProcessor(camControl);

    }

    @Override
    public void render(float delta) {
        camControl.update(delta);

        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT);
        Gdx.gl.glEnable(GL20.GL_DEPTH_TEST);

        cam.update();
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
