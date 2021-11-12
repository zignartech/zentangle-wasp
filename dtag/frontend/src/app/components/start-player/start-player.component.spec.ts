import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StartPlayerComponent } from './start-player.component';

describe('StartPlayerComponent', () => {
  let component: StartPlayerComponent;
  let fixture: ComponentFixture<StartPlayerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ StartPlayerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(StartPlayerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
