package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.SimulationObject;
import com.sun.media.jfxmediaimpl.MediaDisposer;
import org.joml.*;

import java.lang.Math;
import java.nio.FloatBuffer;

/**
 * Created by Fran on 3/14/2018.
 */
public class Renderer implements Disposable{

    private float total;

    private int screenWidth, screenHeight;

    private Camera camera;

    private Shader shader;

    private Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);
    private OpenGLTextureManager openGLTextureManager;

    private Model model;

    private SimulationObject simulationObject, simulationObject2;

    private LightSourceManager lightSourceManager;

    private final Vector3f temp;

    private final Matrix4f combined;


    public Renderer(int screenWidth, int screenHeight) {
        this.screenWidth = screenWidth;
        this.screenHeight = screenHeight;
        this.combined = new Matrix4f();

        openGLTextureManager = new OpenGLTextureManager();

        this.temp = new Vector3f();

        new GLProfiler(Gdx.graphics).enable();
        transformation = new Transformation();

        camera = new Camera(0, 0, 0);

        shader = new Shader(Gdx.files.internal("shaders/default.vert"), Gdx.files.internal("shaders/default.frag"));
        model = new Model(openGLTextureManager, "sphere3.obj", shader, transformation, new Vector3f(), new Vector3f(), 1);
        simulationObject = new SimulationObject(0,0,0,model, 1, "earth");
        simulationObject2 = new SimulationObject(0,0,10000000000f,model, 1000000000f, "sun");

        lightSourceManager = new LightSourceManager(shader, camera, transformation);
        lightSourceManager.addLight(new PointLight(10,0,0));

        shader.use();

        try {
            shader.createUniform("diffuseTex");
            shader.createUniform("projection");
            shader.createUniform("modelView");
            shader.createUniform("og_farPlaneDistance");
            shader.createUniform("u_logarithmicDepthConstant");

        } catch (Exception e) {
            e.printStackTrace();
        }


        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
    }

    public Camera getCamera() {
        return camera;
    }

    public void render(float delta){
        camera.update(delta);
        lightSourceManager.update();

        Gdx.gl.glClearColor(0.0f,0.0f,0.0f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT | Gdx.gl.GL_DEPTH_BUFFER_BIT);

        total += delta*50;

        shader.use();
        Matrix4f projection = transformation.getProjectionMatrix(FOV, screenWidth,screenHeight,1f,10000000f);
        combined.set(projection).mul(transformation.getViewMatrix(camera));

        shader.setFloat("og_farPlaneDistance", 10000000000f);
        shader.setFloat("u_logarithmicDepthConstant", 1f);
        shader.setMat4("projection", projection);


        //this.model.render(camera);
        simulationObject.render(camera);
        simulationObject2.render(camera);
    }

    public Vector3f projectPoint(Vector3f position){
        temp.set(position);
        temp.normalize();
        float res = temp.dot(camera.getFront());
        combined.project(position, new int[]{0,0,screenWidth,screenHeight}, position);

        if(res>0) {
            return position;
        } else {
            return null;
        }
    }


    public void updateScreenSize(int width, int height){
        screenWidth = width;
        screenHeight = height;
    }


    @Override
    public void dispose() {

    }
}
