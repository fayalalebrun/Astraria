package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector3f;
import sun.security.provider.SHA;

/**
 * Created by Fran on 4/10/2018.
 */
public class AtmospherePlanet extends SimulationObject{
    private static final float Kr = 0.0025f, Km = 0.0010f, ESun = 20f, g = -0.95f;


    Sphere atmosphereSphere;
    Shader atmosphereShader;

    float atmosphericRadius, cameraHeight, cameraHeight2, fOuterRadius, fOuterRadius2, fInnerRadius, fInnerRadius2,
            fKrESun, fKmESun, fKr4PI, fKm4PI, fScale, fScaleDepth, fScaleOverScaleDepth, g2;
    final Vector3f waveLength, invWaveLength;

    private final Vector3f lightPosition;

    public AtmospherePlanet(double x, double y, double z, Model model, Shader shader, Shader atmosphereShader, float radius, String name, Renderer renderer) {
        super(x, y, z, model, shader, radius, name);

        this.atmosphereShader = atmosphereShader;

        atmosphereSphere = new Sphere(renderer.getModelManager(),
                renderer.getTransformation(),
                -1,
                false);
        
        atmosphericRadius = radius*1.025f;

        lightPosition = new Vector3f();
        waveLength =  new Vector3f();
        invWaveLength = new Vector3f();
    }

    @Override
    protected void update(Camera cam) {
        super.update(cam);

        atmosphereSphere.setScale(atmosphericRadius);
        atmosphereSphere.setPosition(getPositionRelativeToCamera(cam));
    }

    private void calculateAtmosphereShaderVars(Camera cam){
        lightPosition.set(1,0,0); //temp
        waveLength.set(0.650f, 0.570f, 0.475f);
        invWaveLength.set(1f/(float)Math.pow(waveLength.x,4),1f/(float)Math.pow(waveLength.y,4),
                1f/(float)Math.pow(waveLength.z,4));
        cameraHeight = getPositionRelativeToCamera(cam).length();
        cameraHeight2 = cameraHeight*cameraHeight;
        fOuterRadius = atmosphericRadius;
        fOuterRadius2 = atmosphericRadius * atmosphericRadius;
        fInnerRadius = getRadius();
        fInnerRadius2 = getRadius() * getRadius();
        fKrESun = Kr * ESun;
        fKmESun = Km * ESun;
        fKr4PI = Kr * 4f * (float)Math.PI;
        fKm4PI = Km * 4f * (float)Math.PI;
        fScale = 1f / (fOuterRadius - fInnerRadius);
        fScaleDepth = 1f;
        fScaleOverScaleDepth = fScale/fScaleDepth;
        g2 = g*g;

    }

    private void sendVarsToAtmosphereShader(){
        atmosphereShader.use();
        atmosphereShader.setVec3f("v3LightPos", lightPosition);
        atmosphereShader.setVec3f("v3InvWavelength",invWaveLength);
        atmosphereShader.setFloat("fCameraHeight2",cameraHeight2);
        atmosphereShader.setFloat("fOuterRadius",fOuterRadius);
        atmosphereShader.setFloat("fOuterRadius2",fOuterRadius2);
        atmosphereShader.setFloat("fInnerRadius",fInnerRadius);
        atmosphereShader.setFloat("fKrESun",fKrESun);
        atmosphereShader.setFloat("fKmESun",fKmESun);
        atmosphereShader.setFloat("fKr4PI",fKr4PI);
        atmosphereShader.setFloat("fKm4PI",fKm4PI);
        atmosphereShader.setFloat("fScale",fScale);
        atmosphereShader.setFloat("fScaleDepth",fScaleDepth);
        atmosphereShader.setFloat("fScaleOverScaleDepth",fScaleOverScaleDepth);
        atmosphereShader.setFloat("g",g);
        atmosphereShader.setFloat("g2",g2);
    }

    private void sendVarsToPlanetShader(){
        shader.use();
        shader.setVec3f("v3LightPos", lightPosition);
        shader.setVec3f("v3InvWavelength",invWaveLength);
        shader.setFloat("fCameraHeight2",cameraHeight2);
        shader.setFloat("fOuterRadius",fOuterRadius);
        shader.setFloat("fOuterRadius2",fOuterRadius2);
        shader.setFloat("fInnerRadius",fInnerRadius);
        shader.setFloat("fKrESun",fKrESun);
        shader.setFloat("fKmESun",fKmESun);
        shader.setFloat("fKr4PI",fKr4PI);
        shader.setFloat("fKm4PI",fKm4PI);
        shader.setFloat("fScale",fScale);
        shader.setFloat("fScaleDepth",fScaleDepth);
        shader.setFloat("fScaleOverScaleDepth",fScaleOverScaleDepth);
    }

    @Override
    public void render(Camera cam) {
        calculateAtmosphereShaderVars(cam);

        sendVarsToPlanetShader();

        super.render(cam);



        sendVarsToAtmosphereShader();

        Gdx.gl.glCullFace(Gdx.gl.GL_FRONT);
        atmosphereSphere.render(cam, atmosphereShader);
        Gdx.gl.glCullFace(Gdx.gl.GL_BACK);
    }

    @Override
    public void dispose() {
        super.dispose();
        atmosphereSphere.dispose();
    }
}
