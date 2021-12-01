import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AppMainComponent } from './app.main.component';

describe('App.MainComponent', () => {
  let component: AppMainComponent;
  let fixture: ComponentFixture<AppMainComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [AppMainComponent],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(AppMainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
